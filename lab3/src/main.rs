mod vertex;
mod geometry;
mod program;
mod uniform;
mod camera;
mod index_buffer;
mod consts;
mod drawing;
mod binder;
mod painter;

use glutin;
use gl;
use nalgebra_glm as glm;

use std::rc::Rc;
use std::io::{Write};
use std::collections::HashMap;
use std::default::Default;
use std::time::{Duration, Instant};

use uniform::Uniform;
use drawing::DrawMode;
use camera::Camera;
use painter::Painter;

use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::{WindowBuilder};
use glutin::{Api, GlRequest};
use glutin::dpi::PhysicalPosition;
use crate::uniform::TypedUniform;

// todo: Objects can emit painters which borrow data from them during upload.
//  data must be interpretable as &[VertexAttribute], &[IndexingPrimitive] and perhaps uniforms and programs.
//  assert that all buffer objects contain the same number of atts? what about indexing.
//  move instance_count information to buffer
const GL_VERSION: (u8, u8) = (4, 5);

#[macro_export]
macro_rules! gl_assert_no_err {
    () => {
        assert!(unsafe { gl::GetError() } == gl::NO_ERROR);
    }
}

#[macro_export]
macro_rules! gl_assert {
    ($s:stmt) => {
        $s
        if cfg!(debug_assertions) {
            let err = gl::GetError();
            match err {
                gl::NO_ERROR => {
                },
               _ => {
                    match err {
                        gl::INVALID_ENUM => panic!("GL_INVALID_ENUM"),
                        gl::INVALID_VALUE => panic!("GL_INVALID_VALUE"),
                        gl::INVALID_OPERATION => panic!("GL_INVALID_OPERATION"),
                        gl::INVALID_FRAMEBUFFER_OPERATION => panic!("GL_INVALID_FRAMEBUFFER_OPERATION"),
                        gl::OUT_OF_MEMORY => panic!("GL_OUT_OF_MEMORY"),
                        gl::STACK_UNDERFLOW => panic!("GL_STACK_UNDERFLOW"),
                        gl::STACK_OVERFLOW => panic!("GL_STACK_OVERFLOW"),
                        _ => panic!("unknown error")
                    }
                }
            }
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CoordinateSystem {
    center: glm::Vec3,
    x: glm::Vec4,
    y: glm::Vec4,
    z: glm::Vec4,
}

impl CoordinateSystem {
    const CENTER: glm::Vec3 = glm::Vec3::new(0f32, 0f32, 0f32);
}

#[derive(Debug, Copy, Clone)]
pub struct Scene {
    bg_color: glm::Vec4,
    // skybox: geometry::Cube
}

impl Scene {
    const DARK_GRAY:  glm::Vec3 = glm::Vec3::new(0.23, 0.23, 0.23);
    const LIGHT_BLUE: glm::Vec3 = glm::Vec3::new(0.54, 0.82, 1.0);
}

pub enum Direction {
    Front,
    Back,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Directions {
    pub up: glm::Vec3,
    pub down: glm::Vec3,
    pub front: glm::Vec3,
    pub back: glm::Vec3,
    pub left: glm::Vec3,
    pub right: glm::Vec3,
}

impl Directions {
    const FRONT: glm::Vec3 = glm::Vec3::new( 0f32,  0f32, -1f32);
    const BACK:  glm::Vec3 = glm::Vec3::new( 0f32,  0f32,  1f32);
    const UP:    glm::Vec3 = glm::Vec3::new( 0f32,  1f32,  0f32);
    const DOWN:  glm::Vec3 = glm::Vec3::new( 0f32, -1f32,  0f32);
    const RIGHT: glm::Vec3 = glm::Vec3::new( 1f32,  0f32,  0f32);
    const LEFT:  glm::Vec3 = glm::Vec3::new(-1f32,  0f32,  0f32);
}

fn screen_center(window: &glutin::window::Window) -> PhysicalPosition<u32>{
    let size = window.inner_size();
    PhysicalPosition::new(size.width / 2, size.height / 2)
}

fn center_cursor(window: &glutin::window::Window) {
    let center = screen_center(window);
    window.set_cursor_position(center).unwrap();
}

// todo: update frame rate in the terminal in place.
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("3D labyrinth");

    let gl_context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, GL_VERSION))
        // .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    gl_context.window().set_cursor_visible(false);

    // gl_context.window().set_inner_size(glutin::dpi::LogicalSize::new(400.0, 200.0));
    // gl_context.window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));
    // let size = gl_context.window().inner_size();
    // gl_context.(size);

    let mut camera = Camera::default();
    let labyrinth_grid_size = 5;
    let light_direction = Directions::DOWN + Directions::RIGHT + Directions::FRONT;

    let labyrinth_uniforms = [
        ("perspective_matrix", Box::new(camera.perspective_matrix().as_ref().clone()) as _),
        ("view_matrix", Box::new(camera.view_matrix().as_ref().clone()) as _),
    ].into_iter();
    let skybox_uniforms = [
        ("perspective_matrix", Box::new(camera.perspective_matrix().as_ref().clone()) as _),
        ("view_matrix", Box::new(camera.view_matrix().as_ref().clone()) as _),
    ].into_iter();
    let sphere_uniforms = [
        ("perspective_matrix", Box::new(camera.perspective_matrix().as_ref().clone()) as _),
        ("view_matrix", Box::new(camera.view_matrix().as_ref().clone()) as _),
        ("light_direction", Box::new(light_direction.as_ref().clone()) as _)
    ].into_iter();

    let mut labyrinth_painter = Painter::new(geometry::labyrinth(labyrinth_uniforms, labyrinth_grid_size), DrawMode::Triangles)
        .instanced(labyrinth_grid_size * labyrinth_grid_size * labyrinth_grid_size);
    let mut skybox_painter = Painter::new(geometry::cube(skybox_uniforms), DrawMode::Triangles);
    let mut sphere_painter = Painter::new(geometry::sphere(sphere_uniforms), DrawMode::Triangles);

    // todo: should all painters get perspective and view matrices?

    const PERSPECTIVE_UNIFORM: &str = "perspective";
    const VIEW_UNIFORM: &str = "view";
    const LIGHT_UNIFORM: &str = "light";

    let uniform_name_location_mapping = HashMap::from([
        (PERSPECTIVE_UNIFORM, 0),
        (VIEW_UNIFORM, 1),
        (LIGHT_UNIFORM, 2),
    ]);

    gl_assert_no_err!();
    unsafe { gl::Enable(gl::DEPTH_TEST); }
    gl_assert_no_err!();

    let mut mouse_pos: Option<PhysicalPosition<f64>> = None;

    let mut frame_rate_display = Instant::now();
    let mut last_mouse_input = Instant::now();

    // todo: make mouse cursor stick to middle of the screen.

    let mut show_cursor = false;
    let screen_center = |window: &glutin::window::Window| {
        let size = window.inner_size();
        PhysicalPosition::new(size.width / 2, size.height / 2)
    };

    let mut draw_mode = drawing::DrawMode::Triangles;
    let mut key_counter = 0;

    let mut fps_counter = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // todo: for smoother movement and better frame rates process all inputs once per each frame.
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        if let Some(old_pos) = mouse_pos {
                            if last_mouse_input + Duration::from_millis(16) < Instant::now() {
                                last_mouse_input = Instant::now();
                                let x_diff = old_pos.x - position.x;
                                let y_diff = old_pos.y - position.y;
                                mouse_pos = Some(position);

                                camera.rotate( (y_diff as f32).to_radians(), (x_diff as f32).to_radians());
                                skybox_painter.binder_mut().update_uniform(
                                    "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                                );
                                sphere_painter.binder_mut().update_uniform(
                                    "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                                );
                                labyrinth_painter.binder_mut().update_uniform(
                                    "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                                );
                            }
                        } else {
                            mouse_pos = Some(position);
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match keycode {
                                VirtualKeyCode::A => camera.r#move(&Direction::Left),
                                VirtualKeyCode::D => camera.r#move(&Direction::Right),
                                VirtualKeyCode::Q => camera.r#move(&Direction::Up),
                                VirtualKeyCode::Z => camera.r#move(&Direction::Down),
                                VirtualKeyCode::W => camera.r#move(&Direction::Front),
                                VirtualKeyCode::C => {
                                    key_counter += 1;
                                    if key_counter % 2 == 1 {
                                        if draw_mode == drawing::DrawMode::Triangles {
                                            draw_mode = drawing::DrawMode::LineLoop;
                                        } else {
                                            draw_mode = drawing::DrawMode::Triangles;
                                        }
                                    }
                                }

                                VirtualKeyCode::S => camera.r#move(&Direction::Back),
                                VirtualKeyCode::Escape => {
                                    show_cursor = !show_cursor;
                                    gl_context.window().set_cursor_visible(show_cursor)
                                },
                                _ => (),
                            };
                            skybox_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                            );
                            sphere_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                            );
                            labyrinth_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(camera.view_matrix().as_ref().clone()),
                            );
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(Scene::LIGHT_BLUE.x, Scene::DARK_GRAY.y, Scene::LIGHT_BLUE.z, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
        fps_counter += 1;
        if frame_rate_display + Duration::from_millis(500) < Instant::now() {
            frame_rate_display = Instant::now();
            print!("\r{} fps", fps_counter * 2);
            std::io::stdout().flush().unwrap();
            fps_counter = 0;
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
        labyrinth_painter.update_draw_mode(draw_mode);
        sphere_painter.update_draw_mode(draw_mode);
        skybox_painter.update_draw_mode(draw_mode);

        labyrinth_painter.draw();
        sphere_painter.draw();
        skybox_painter.draw();
        gl_context.swap_buffers().unwrap();
    });
}

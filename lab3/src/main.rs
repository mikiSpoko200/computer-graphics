mod vertex;
mod geometry;
mod program;
mod uniform;
mod camera;
mod index_buffer;
mod drawing;
mod binder;
mod painter;
mod colliders;

use glutin;
use gl;
use nalgebra_glm as glm;

use std::io::{Write};
use std::default::Default;
use std::time::{Duration, Instant};

use drawing::DrawMode;
use camera::Camera;
use painter::Painter;

use glutin::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::{WindowBuilder};
use glutin::{Api, GlRequest};
use glutin::dpi::PhysicalPosition;
use crate::camera::{CameraPerspectiveState, CameraViewState, FixedMovable, FreeRoamingCamera, KinematicCamera, PerspectiveMatrixProvider, Rotatable, ViewMatrixProvider};

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

    let perspective = CameraPerspectiveState::default();
    let view = CameraViewState::default();
    let mut free_roam_cam = FreeRoamingCamera::from(Camera::new(perspective.clone(), view));
    let mut hero_cam = camera::HeroShotCamera::new(perspective, Directions::FRONT, CoordinateSystem::CENTER, 4.0);

    let labyrinth_grid_size = 5;
    let light_direction = Directions::DOWN + Directions::RIGHT + Directions::FRONT;

    const PERSPECTIVE_MATRIX_ID: &str = "perspective_matrix";
    const VIEW_MATRIX_ID: &str = "view_matrix";
    const LIGHT_DIRECTION_ID: &str = "light_direction";
    const PLAYER_POSITION_ID: &str = "player_position";
    const GRID_SIZE_ID: &str = "grid_size";

    let mut labyrinth_uniforms = uniform::to_owned([
        (PERSPECTIVE_MATRIX_ID, free_roam_cam.perspective_matrix()),
        (VIEW_MATRIX_ID, free_roam_cam.view_matrix()),
    ]).collect::<Vec<_>>();
    let skybox_uniforms = uniform::to_owned([
        (PERSPECTIVE_MATRIX_ID, free_roam_cam.perspective_matrix()),
        (VIEW_MATRIX_ID, free_roam_cam.view_matrix()),
    ]);
    let mut sphere_uniforms = uniform::to_owned([
        (PERSPECTIVE_MATRIX_ID, free_roam_cam.perspective_matrix()),
        (VIEW_MATRIX_ID, free_roam_cam.view_matrix()),
    ]).collect::<Vec<_>>();
    sphere_uniforms.push((LIGHT_DIRECTION_ID, Box::new(light_direction.as_ref().clone())));
    sphere_uniforms.push((PLAYER_POSITION_ID, Box::new(free_roam_cam.get_position().as_ref().clone())));
    labyrinth_uniforms.push((GRID_SIZE_ID, Box::new(labyrinth_grid_size as f32) as _));

    let mut labyrinth_painter = Painter::new(geometry::labyrinth(labyrinth_uniforms.into_iter(), labyrinth_grid_size), DrawMode::Triangles)
        .instanced(labyrinth_grid_size * labyrinth_grid_size * labyrinth_grid_size);
    let mut skybox_painter = Painter::new(geometry::cube(skybox_uniforms), DrawMode::Triangles);
    let mut sphere_painter = Painter::new(geometry::sphere(sphere_uniforms.into_iter()), DrawMode::Triangles);

    gl_assert_no_err!();
    unsafe { gl::Enable(gl::DEPTH_TEST); }
    gl_assert_no_err!();

    let mut frame_rate_display = Instant::now();
    let mut use_free_roaming_camera = true;
    let mut draw_mode = DrawMode::Triangles;

    const FREE_ROAM_CAM: usize = 0;
    const HERO_CAM: usize = 1;
    let mut current_cam = FREE_ROAM_CAM;

    let mut fps_counter = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // todo: for smoother movement and better frame rates process all inputs once per each frame.

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            },
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::MouseMotion { delta: (y_delta, x_delta) } => {
                        let current_camera: &mut dyn KinematicCamera = if current_cam == FREE_ROAM_CAM { &mut free_roam_cam } else { &mut hero_cam };
                        current_camera.rotate( (x_delta as f32).to_radians(), (-y_delta as f32).to_radians());
                        skybox_painter.binder_mut().update_uniform(
                            "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                        );
                        sphere_painter.binder_mut().update_uniform(
                            "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                        );
                        labyrinth_painter.binder_mut().update_uniform(
                            "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                        );
                    }
                    DeviceEvent::Key(KeyboardInput{ state: ElementState::Pressed, virtual_keycode: Some(key_code), .. }) => {
                        match key_code {
                            VirtualKeyCode::A => free_roam_cam.fixed_move(&Direction::Left),
                            VirtualKeyCode::D => free_roam_cam.fixed_move(&Direction::Right),
                            VirtualKeyCode::Q => free_roam_cam.fixed_move(&Direction::Up),
                            VirtualKeyCode::Z => free_roam_cam.fixed_move(&Direction::Down),
                            VirtualKeyCode::W => free_roam_cam.fixed_move(&Direction::Front),
                            VirtualKeyCode::S => free_roam_cam.fixed_move(&Direction::Back),
                            VirtualKeyCode::L => hero_cam.fixed_move(&Direction::Front),
                            VirtualKeyCode::K => hero_cam.fixed_move(&Direction::Back),
                            VirtualKeyCode::C => {
                                if draw_mode == DrawMode::Triangles {
                                    draw_mode = DrawMode::LineLoop;
                                } else {
                                    draw_mode = DrawMode::Triangles;
                                }
                            }
                            VirtualKeyCode::Escape => {
                                if current_cam == FREE_ROAM_CAM {
                                    current_cam = HERO_CAM;
                                } else {
                                    current_cam = FREE_ROAM_CAM;
                                }
                            },
                            _ => (),
                        };
                        {
                            let pos = free_roam_cam.get_position().as_ref().clone();
                            let current_camera: &mut dyn KinematicCamera = if current_cam == FREE_ROAM_CAM { &mut free_roam_cam } else { &mut hero_cam };
                            skybox_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                            );
                            sphere_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                            );
                            sphere_painter.binder_mut().update_uniform(
                                PLAYER_POSITION_ID, Box::new(pos)
                            );
                            labyrinth_painter.binder_mut().update_uniform(
                                "view_matrix", Box::new(current_camera.view_matrix().as_ref().clone()),
                            );

                        }

                    }
                    _ => (),
                }
            }
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

        labyrinth_painter.draw();
        if current_cam != FREE_ROAM_CAM {
            sphere_painter.draw();
        }
        skybox_painter.draw();
        gl_context.swap_buffers().unwrap();
    });
}

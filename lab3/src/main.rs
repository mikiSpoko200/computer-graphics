extern crate alloc;

mod vertex;
mod geometry;
mod program;
mod uniform;
mod camera;
mod index_buffer;
mod consts;
mod drawing;

use alloc::rc::Rc;
use std::collections::HashMap;
use std::default::Default;
use std::time::{Duration, Instant};
use nalgebra_glm as glm;

use uniform::Uniform;
use program::Program;
use drawing::DrawMode;
use index_buffer::{IndexBuffer, IndexingMode, IndexType, IndexBufferObject};
use vertex::{VertexAttribute, BufferObject};

use glutin;
use gl;

use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::{Api, GlRequest};
use glutin::dpi::PhysicalPosition;
use crate::camera::Camera;

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

pub struct Painter<I: IndexBuffer> {
    binder: Binder<I>,
    draw_mode: DrawMode,
    instance_count: Option<usize>,
}

impl<I: IndexBuffer> Painter<I> {
    pub fn new(binder: Binder<I>, draw_mode: DrawMode) -> Self {
        Self {
            binder,
            draw_mode,
            instance_count: None
        }
    }

    pub fn binder(&self) -> &Binder<I> {
        &self.binder
    }

    pub fn binder_mut(&mut self) -> &mut Binder<I> { &mut self.binder }

    pub fn instanced(mut self, instance_count: usize) -> Self {
        self.instance_count = Some(instance_count);
        self
    }

    pub fn update_draw_mode(&mut self, new: DrawMode) {
        self.draw_mode = new;
    }

    pub fn draw(&self) {
        let _draw_scoped_binder = self.binder.draw_binder();
        match (self.instance_count, self.binder.index_type()) {
            (Some(instance_count), Some(ref index_type)) => {
                drawing::instanced::draw_indexed(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    index_type,
                    instance_count
                );
            },
            (Some(instance_count), None) => {
                drawing::instanced::draw_arrays(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    instance_count
                );
            },
            (None, Some(ref index_type)) => {
                drawing::draw_indexed(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    index_type
                );
            },
            (None, None) => {
                drawing::draw_arrays(&self.draw_mode, self.binder.vertex_count());
            }
        }
    }
}

// fixme: attribute / uniform layout provider - as of now layouts are specified in order.
//      quick solution -> print the manifest of (current layout - glsl lifetime - name)?

pub struct Binder<I>
where
    I: IndexBuffer
{
    vao: vertex::ArrayObject,
    vbos: Vec<Box<dyn vertex::Buffer>>,
    ebo: IndexingMode<I>,
    program: Program,
    uniforms: Vec<Box<dyn Uniform>>,
}

impl<I> Binder<I> where I: IndexBuffer,
{
    pub fn new(
        vbos: Vec<Box<dyn vertex::Buffer>>,
        ebo: IndexingMode<I>,
        program: Program,
        uniforms: Vec<Box<dyn Uniform>>,
    ) -> Self {
        let vao = vertex::ArrayObject::create();
        Self { vao, vbos, ebo, program, uniforms, }
    }

    // todo: scoped_binder controls if appropriate object is already bound if so it returns null binder of sort.
    //      uniform indexes or more generally should be provided and managed by and external object.

    pub fn bind_uniforms(&self) {
        println!("Binding uniforms");
        let _program_binder = self.program_binder();
        for (index, uniform) in self.uniforms.iter().enumerate() {
            uniform.bind(index as _);
        }
    }

    pub fn add_uniform(&mut self, index: usize, uniform: Box<dyn Uniform>) {
        // todo: index is never stored and depends on the order in uniforms - this is terrible fix it please xoxo
        if self.uniforms.len() > index {
            panic!("uniform with index {} already exists", index);
        } else {
            self.uniforms.push(uniform)
        }
    }

    pub fn update_uniform(&mut self, index: usize, uniform: Box<dyn Uniform>) {
        self.uniforms[index] = uniform;
        {
            let _program_binder = self.program_binder();
            self.uniforms[index].as_ref().bind(index as _);
        }
    }

    pub fn upload(&mut self) {
        let _program_scoped_binder = self.program.scoped_binder();
        // for (index, uniform) in self.uniforms.iter().enumerate() {
        //     pritnln!("Binding uniforms");
        //     uniform.as_ref().bind(index as _);
        // }

        let _vao_binder = self.vao.scoped_binder();
        for (index, vbo) in self.vbos.iter().enumerate() {
            let _scoped_binder = vbo.as_ref().scoped_binder();
            gl_assert_no_err!();
            vbo.as_ref().upload();
            gl_assert_no_err!();
            self.vao.set_vertex_attrib_pointer(index as _, &vbo.as_ref().attribute_type());
            gl_assert_no_err!();
        }

        if let Some(ref index_buffer) = self.ebo {
            let _ebo_binder = index_buffer.scoped_binder();
            index_buffer.upload();
        }
    }

    pub fn vertex_count(&self) -> usize {
        if let Some(ref index_buffer) = self.ebo {
            index_buffer.vertex_count()
        } else {
            self.vbos.first().unwrap().as_ref().vertex_count()
        }
    }

    pub(self) fn vao_binder(&self) -> vertex::array_object::ScopedBinder {
        self.vao.scoped_binder()
    }

    pub fn index_type(&self) -> Option<IndexType> {
        self.ebo.as_ref().map(|index_buffer| index_buffer.index_type())
    }

    pub(self) fn program_binder(&self) -> program::ScopedBinder { self.program.scoped_binder() }

    pub fn draw_binder(&self) -> DrawScopedBinder {
        DrawScopedBinder::new(self.program_binder(), self.vao_binder())
    }
}

pub struct DrawScopedBinder(program::ScopedBinder, vertex::array_object::ScopedBinder);

impl DrawScopedBinder {
    pub fn new(program: program::ScopedBinder, vao: vertex::array_object::ScopedBinder) -> Self {
        Self(program, vao)
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
    const FRONT: glm::Vec3 = glm::Vec3::new( 0f32,  0f32,  1f32);
    const BACK:  glm::Vec3 = glm::Vec3::new( 0f32,  0f32, -1f32);
    const UP:    glm::Vec3 = glm::Vec3::new( 0f32,  1f32,  0f32);
    const DOWN:  glm::Vec3 = glm::Vec3::new( 0f32, -1f32,  0f32);
    const RIGHT: glm::Vec3 = glm::Vec3::new( 1f32,  0f32,  0f32);
    const LEFT:  glm::Vec3 = glm::Vec3::new(-1f32,  0f32,  0f32);
}

struct Triangle {
    binder: Binder<IndexBufferObject<u8>>
}

impl Triangle {
    pub fn new() -> Self {
        let triangle= attributes!(
            (-0.5, 0.0),
            ( 0.5, 0.0),
            ( 0.0, 0.8)
        );

        let colors = attributes!(
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0f32)
        );

        let scales = attributes!(
            0.5,
            0.5,
            0.5f32
        );

        let offsets = attributes!(
            (0.0, 0.5),
            (-0.5, -0.2),
            (0.5, -0.2f32),
        );

        let indices = vec!(1, 2, 3u8).into_boxed_slice();
        let index_buf = IndexBufferObject::create(indices);

        let program = Program::from_file(
            "shaders/triangle_v.glsl".as_ref(),
            "shaders/triangle_f.glsl".as_ref()
        );
        let mut binder = Binder::new(
            vec!(
                Box::new(triangle),
                Box::new(colors),
                Box::new(scales),
                Box::new(offsets),
            ),
            Some(index_buf),
            program,
            Vec::new()
        );

        binder.upload();
        {
            let _vao_binder = binder.vao_binder();
            binder.vao.set_attrib_divisor(2, 1);
            binder.vao.set_attrib_divisor(3, 1);
        }

        Self { binder }
    }
}

pub fn sp(radius: f32, poly_count: usize) -> (Box<[VertexAttribute<f32, 3>]>, Box<[VertexAttribute<f32, 3>]>, Box<[u16]>) {
    use std::f32::consts::PI;

    let mut vertices = Vec::new();
    let mut normals = Vec::new();

    let sector_angle_offset = 2.0 * PI / poly_count as f32;
    let stack_angle_offset = PI / poly_count as f32;

    for stack_index in 0..=poly_count {
        let stack_angle = PI / 2.0 - stack_index as f32 * stack_angle_offset;
        let xy = radius * f32::cos(stack_angle);
        let  z = radius * f32::sin(stack_angle);

        for sector_index in 0..=poly_count {
            let sector_angle = sector_index as f32 * sector_angle_offset;
            let x = xy * f32::cos(sector_angle);
            let y = xy * f32::sin(sector_angle);
            let point = glm::Vec3::new(x, y, z);

            vertices.push(VertexAttribute::from(*point.as_ref()));
            normals.push(VertexAttribute::from(*(point / radius).as_ref()));
        }
    }

    let mut indices = Vec::new();
    for stack_index in 0..poly_count {
        let k1 = (stack_index * (poly_count + 1)) as u16;
        let k2 = (k1 as usize + poly_count + 1) as u16;

        for (k1, k2) in (k1..).zip(k2..).take(poly_count) {
            if stack_index != 0 {
                indices.push(k1);
                indices.push(k2);
                indices.push(k1 + 1);
            }
            if stack_index != poly_count - 1 {
                indices.push(k1 + 1);
                indices.push(k2);
                indices.push(k2 + 1);
            }
        }
    }

    (vertices.into_boxed_slice(), normals.into_boxed_slice(), indices.into_boxed_slice())
}

pub fn sphere() -> Binder<IndexBufferObject<u16>> {
    let (vertices, normals, indices) = sp(1.0, 25);

    let positions = Box::new(BufferObject::create(vertices));
    let normals = Box::new(BufferObject::create(normals));
    let index_buf = IndexBufferObject::create(indices);

    let program = Program::from_file(
        "shaders/sphere_v.glsl".as_ref(),
        "shaders/sphere_f.glsl".as_ref()
    );

    let mut binder = Binder::new(
        vec!(positions, normals),
        Some(index_buf),
        program,
        vec!()
    );
    binder.upload();
    binder
}

pub fn template_triangle(a: f32) -> [glm::Vec3; 3] {
    let radius = a / f32::sqrt(3.0);
    [
        glm::vec3(0.0, radius, 0.0), // top point
        glm::vec3(-a / 2.0, -radius / 2.0, 0.0),
        glm::vec3(a / 2.0, -radius / 2.0, 0.0)
    ]
}

pub fn labyrinth(n: usize) -> Binder<IndexBufferObject> {
    let scale = 1.0 / n as f32;
    let tail_center_offset = glm::vec3(1f32, 1f32, 1f32) / (2.0 * n as f32);

    let scaled_model = template_triangle(2.0).into_iter().map(|position| {
        position * scale
    });
    let mut positions: Vec<VertexAttribute<f32, 3>> = Vec::new();

    for xi in 0..n {
        for yi in 0..n {
            for zi in 0..n {
                let corner_offset = glm::vec3(xi as _, yi as _, zi as _) / n as f32;
                let center_offset = corner_offset + tail_center_offset;
                let center_offset_ndc = 2.0 * center_offset - glm::vec3(1.0, 1.0, 1.0);
                for position in scaled_model.clone() {
                    let arr = (position + center_offset_ndc).as_ref().clone();
                    positions.push(VertexAttribute::from(arr));
                }
            }
        }
    }

    let buffer_object = BufferObject::create(positions.into_boxed_slice());

    let program = Program::from_file(
        "shaders/labyrinth_v.glsl".as_ref(),
        "shaders/labyrinth_f.glsl".as_ref()
    );

    let mut binder = Binder::new(vec!(Box::new(buffer_object)), None, program, vec!());
    binder.upload();
    binder
}

// todo: update framerate in the terminal in place.
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    // let aspect_ratio = window.

    let gl_context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, GL_VERSION))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut camera = Camera::default();

    // gl_context.window().set_inner_size(glutin::dpi::LogicalSize::new(400.0, 200.0));
    // gl_context.window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));
    // let size = gl_context.window().inner_size();
    // gl_context.(size);

    let mut ball_painter = Painter::new(sphere(), DrawMode::Triangles);
    let light_direction = Directions::DOWN + Directions::RIGHT + Directions::FRONT;
    let grid_size = 5;
    let mut labyrinth_painter = Painter::new(labyrinth(grid_size), DrawMode::Triangles).instanced(grid_size * grid_size * grid_size);

    const PERSPECTIVE_UNIFORM: &str = "perspective";
    const VIEW_UNIFORM: &str = "view";
    const LIGHT_UNIFORM: &str = "light";

    let uniform_name_location_mapping = HashMap::from([
        (PERSPECTIVE_UNIFORM, 0),
        (VIEW_UNIFORM, 1),
        (LIGHT_UNIFORM, 2),
    ]);

    ball_painter.binder_mut().add_uniform(
        0, Box::new(camera.perspective_matrix().as_ref().clone())
    );
    ball_painter.binder_mut().add_uniform(
        1, Box::new(camera.view_matrix().as_ref().clone())
    );
    ball_painter.binder_mut().add_uniform(
        2, Box::new(light_direction.as_ref().clone())
    );
    ball_painter.binder().bind_uniforms();

    labyrinth_painter.binder_mut().add_uniform(
        0, Box::new(camera.perspective_matrix().as_ref().clone())
    );
    labyrinth_painter.binder_mut().add_uniform(
        1, Box::new(camera.view_matrix().as_ref().clone())
    );
    labyrinth_painter.binder().bind_uniforms();

    gl_assert_no_err!();
    unsafe { gl::Enable(gl::DEPTH_TEST); }
    gl_assert_no_err!();

    let mut mouse_pos: Option<PhysicalPosition<f64>> = None;

    let mut blast_disp_time = std::time::Instant::now();
    let mut last_mouse_input = std::time::Instant::now();

    // todo: make mouse cursor stick to middle of the screen.

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        if let Some(old_pos) = mouse_pos {
                            if last_mouse_input + Duration::from_millis(33) < Instant::now() {
                                last_mouse_input = Instant::now();
                                let x_diff = old_pos.x - position.x;
                                let y_diff = old_pos.y - position.y;
                                mouse_pos = Some(position);
                                // camera.fixed_rotate_right();

                                camera.rotate( (y_diff as f32).to_radians(), (x_diff as f32).to_radians());
                                ball_painter.binder_mut().update_uniform(
                                    uniform_name_location_mapping[VIEW_UNIFORM],
                                    Box::new(camera.view_matrix().as_ref().clone()),
                                );
                                labyrinth_painter.binder_mut().update_uniform(
                                    uniform_name_location_mapping[VIEW_UNIFORM],
                                    Box::new(camera.view_matrix().as_ref().clone()),
                                );
                            }
                        } else {
                            mouse_pos = Some(position);
                        }
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match keycode {
                                VirtualKeyCode::A => camera.move_(Directions::LEFT * 0.01),
                                VirtualKeyCode::D => camera.move_(Directions::RIGHT * 0.01),
                                VirtualKeyCode::Q => camera.move_(Directions::UP * 0.01),
                                VirtualKeyCode::Z => camera.move_(Directions::DOWN * 0.01),
                                VirtualKeyCode::W => camera.move_(Directions::FRONT * 0.01),
                                VirtualKeyCode::S => camera.move_(Directions::BACK * 0.01),
                                _ => (),
                            };
                            ball_painter.binder_mut().update_uniform(
                                1, Box::new(camera.view_matrix().as_ref().clone())
                            );
                            labyrinth_painter.binder_mut().update_uniform(
                                1, Box::new(camera.view_matrix().as_ref().clone())
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
        if blast_disp_time + Duration::from_millis(500) < Instant::now() {
            blast_disp_time = Instant::now();
            println!("Camera position: ({}, {}, {})", camera.view_matrix()[12], camera.view_matrix()[13], camera.view_matrix()[14]);
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }
        labyrinth_painter.draw();
        ball_painter.draw();
        gl_context.swap_buffers().unwrap();
    });
}

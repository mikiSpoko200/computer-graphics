mod vertex;
mod program;
mod uniform;

use uniform::Uniform;
use program::Program;
use vertex::Attribute;

use glutin;
use gl;
use gl::types::*;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::{Api, GlRequest};

const GL_VERSION: (u8, u8) = (3, 3);

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

pub struct Binder {
    vao: vertex::ArrayObject,
    vbos: Vec<Box<dyn vertex::Buffer>>,
    #[allow(unused)]
    ebo: GLuint,
    program: Program,
    uniforms: Vec<Box<dyn Uniform>>
}

impl Binder {
    pub fn new(vbos: Vec<Box<dyn vertex::Buffer>>, uniforms: Vec<Box<dyn Uniform>>,  program: Program) -> Self {
        let vao = vertex::ArrayObject::create();
        let ebo = 0;
        Self { vao, vbos, ebo, program, uniforms }
    }

    pub fn upload(&mut self) {

        // fixme: attribute / uniform layout provider - as of now layouts are specified in order.
        //      quick solution -> print the manifest of (current layout - glsl lifetime - name)?

        let _program_scoped_binder = self.program.scoped_binder();
        for (index, uniform) in self.uniforms.iter().enumerate() {
            uniform.bind(index as _);
        }

        let _vao_binder = self.vao.scoped_binder();
        for (index, vbo) in self.vbos.iter().enumerate() {
            let _scoped_binder = vbo.scoped_binder();

            vbo.upload();
            self.vao.set_vertex_attrib_pointer(index as _, &vbo.attr_type(), &_vao_binder);
        }
    }

    pub(self) fn vao_binder(&self) -> vertex::array_object::ScopedBinder {
        self.vao.scoped_binder()
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


pub fn draw(vertex_count: usize) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, vertex_count as _);
    }
}

struct Triangle {
    binder: Binder
}

macro_rules! attributes {
    () => {
        Vec::new().into_boxed_slice();
    };
    ($elem: expr; $n: expr) => (
        $crate::std::Vec::from_iter(
            $crate::iter::repeat($elem).take($n)
        ).into_boxed_slice()
    );
    ($($x:expr),+ $(,)?) => {
        vec!($( $crate::vertex::AttributeArray::from($x)),+).into_boxed_slice()
    };
}

impl Triangle {
    pub fn new() -> Self {
        let triangle = attributes!(
            (-0.5, 0.0),
            ( 0.5, 0.0), 
            ( 0.0, 0.8f32)
        );

        let colors = attributes!(
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0f32)
        );

        let trig_buffer = vertex::BufferObject::create(
            &triangle, <(f32, f32) as Attribute>::get_type()
        );

        let color_buffer = vertex::BufferObject::create(
            &colors, <(f32, f32, f32) as Attribute>::get_type()
        );

        let program = Program::from_file(
            "shaders/triangle_v.glsl".as_ref(), 
            "shaders/triangle_f.glsl".as_ref()
        );
        let mut binder = Binder::new(vec!(Box::new(trig_buffer), Box::new(color_buffer)), Vec::new(), program);
        binder.upload();
        Self {
            binder
        }
    }

    pub fn draw(&self) {
        let _draw_binder = self.binder.draw_binder();

        draw(3);
    }
}

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

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

    let tirangle = Triangle::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; 
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
        unsafe {
            gl::ClearColor(0.0, 0.3, 0.3, 1.0);
        }
        tirangle.draw();
    });
}

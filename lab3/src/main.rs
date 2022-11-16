use glutin;
use gl;
use gl::BUFFER;
use gl::types::*;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};
use glutin::event::VirtualKeyCode::B;

const GL_VERSION: (u8, u8) = (3, 3);

trait GlData<D: GlPrimitive = f32> {
    fn byte_size(&self) -> usize;

    fn member_size(&self) -> usize {
        std::mem::sizeof::<D>()
    }
}

trait GlPrimitive {}

macro_rules! primitive {
    ($($_type:ident), +) => {
        impl GlPrimitive for $_type {}
    }
}

primitive!(f32, i32);

impl<T> GlData for T where T: AsRef<[GlData::Data]> {
    type Data = f32;

    fn byte_size(&self) -> usize {
        self.as_ref().len() * self.member_size()
    }
}

pub mod shaders {
    use gl;
    use std;
    use std::ffi::{CString, CStr};

    pub struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
            let id = shader_from_source(source, kind)?;
            Ok(Shader { id })
        }

        pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::VERTEX_SHADER)
        }

        pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::FRAGMENT_SHADER)
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }

    fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(id)
    }

    fn create_whitespace_cstring_with_len(len: usize) -> CString {
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len));
        // convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    struct ProgramCtx<'program>(&'program Program);

    impl<'program> ProgramCtx<'_> {
        pub fn new(program: &'_ Program) -> Self {
            unsafe { gl::UseProgram(program.id()) }
            Self(program)
        }
    }

    impl Drop for ProgramCtx<'_> {
        fn drop(&mut self) {
            unsafe { gl::UseProgram(0) }
        }
    }

    pub struct Program {
        id: gl::types::GLuint,
    }

    impl Program {
        pub fn from_file(vertex: &std::path::Path, fragment: &std::path::Path) -> Self {
            let v_code = std::fs::read_to_string(vertex).unwrap().into;
            let f_code = std::fs::read_to_string(fragment).unwrap();

            let v_code_raw = CString::new(v_code).unwrap();
            let f_code_raw = CString::new(f_code).unwrap();

            let v_shader = Shader::from_source(v_code_raw.as_ref(), gl::VERTEX_SHADER).unwrap();
            let f_shader = Shader::from_source(f_code_raw.as_ref(), gl::FRAGMENT_SHADER).unwrap();

            Self::new(&v_shader, &f_shader)
        }

        pub fn from_shaders<T: AsRef<Shader>>(shaders: &[T]) -> Result<Program, String> {
            let program_id = unsafe { gl::CreateProgram() };

            for shader in shaders {
                unsafe { gl::AttachShader(program_id, shader.id()); }
            }

            unsafe { gl::LinkProgram(program_id); }

            let mut success: gl::types::GLint = 1;
            unsafe {
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            }

            if success == 0 {
                let mut len: gl::types::GLint = 0;
                unsafe {
                    gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = create_whitespace_cstring_with_len(len as usize);

                unsafe {
                    gl::GetProgramInfoLog(
                        program_id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar
                    );
                }

                return Err(error.to_string_lossy().into_owned());
            }

            for shader in shaders {
                unsafe { gl::DetachShader(program_id, shader.id()); }
            }

            Ok(Program { id: program_id })
        }

        fn new(v_shader: &Shader, f_shader: &Shader) -> Self {
            let program = Program::from_shaders(&[v_shader, f_shader]).unwrap();

            Self {
                id: program.id
            }
        }

        pub fn use_ctx(&self) -> ProgramCtx<'_> {
            ProgramCtx::new(self)
        }

        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }
    }

    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}

use shaders::{Shader, Program};

struct BufferObject<Data, Primitive>
where Data: GlData<Primitive>, Primitive: GlPrimitive
{
    id: GLuint,
    data: D
}

impl<Data, Primitive> BufferObject<Data, Primitive>
where Data: GlData<Primitive>, Primitive: GlPrimitive
{
    pub fn create(data: Data) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffer(&mut id);
        }


        Self {}
    }

    pub fn with_binded() -> BufferObjectCtx {

    }
}

struct BufferObjectCtx(GLuint);

impl BufferObjectCtx {
    pub fn new(buffer_id: GLuint) -> Self {
        unsafe { gl::BindBuffer(gl::VERTEX_ARRAY, buffer_id) }
        Self(buffer_id)
    }
}

impl Drop for BufferObjectCtx {
    fn drop(&mut self) {
        unsafe { gl::BindBuffer(gl::VERTEX_ARRAY, 0); }
    }
}

struct VertexAttributeObjectCtx(GLuint);

impl VertexAttributeObjectCtx {
    pub fn new(vao_id: GLuint) -> Self {
        unsafe { gl::BindVertexArray(vao_id); }
        Self(vao_id)
    }
}

impl Drop for VertexAttributeObjectCtx {
    fn drop(&mut self) {
        unsafe { gl::BindVertexArray(0); }
    }
}

struct GlBinder<'data, D: GlPrimitive> {
    vbos: Vec<BufferObject<D>>,
    vao: GLuint,
    ebo: GLuint,
    program: Program,
    uniforms: Vec<&'data dyn GlData<D>>
}

impl<'data> GlBinder<'_> {
    pub fn new<const BUFFER_COUNT: GLuint>() -> Self {
        let mut vbos = vec!(0, BUFFER_COUNT);
        let mut vao = 0;

        unsafe {
            if BUFFER_COUNT > 0 {
                gl::CreateBuffers(BUFFER_COUNT as _, vbos.as_mut_ptr())
            }
            gl::CreateVertexArrays(1, &mut vao);
        }

        Self {

        }
    }
}

struct Triangle {
    vbos: Vec<GLuint>,
    vao: GLuint,
    program: GLuint
}



impl Triangle {
    pub fn new() -> Self {
        gl::CreateVertexArrays()
        gl::CreateBuffers()
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
    });
}

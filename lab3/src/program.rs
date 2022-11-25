use gl;
use gl::types::GLuint;
use std::ffi::{CString, CStr};

pub struct Shader {
    id: GLuint,
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

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        println!("Destroying shader");
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

pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(program_id: GLuint) -> Self {
        println!("Binding program {program_id}");
        unsafe { gl::UseProgram(program_id); }
        loop {
            let err = unsafe { gl::GetError() };
            if err == gl::NO_ERROR {
                break;
            }
            println!("Error: {:?}", err);
        }
        Self(program_id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        println!("Unbinding program {}", self.0);
        unsafe { gl::UseProgram(0) }
    }
}

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_file(vertex: &std::path::Path, fragment: &std::path::Path) -> Self {
        let v_code = std::fs::read_to_string(vertex).unwrap();
        let f_code = std::fs::read_to_string(fragment).unwrap();

        let v_code_raw = CString::new(v_code).unwrap();
        let f_code_raw = CString::new(f_code).unwrap();

        let v_shader = Shader::from_source(v_code_raw.as_ref(), gl::VERTEX_SHADER).unwrap();
        let f_shader = Shader::from_source(f_code_raw.as_ref(), gl::FRAGMENT_SHADER).unwrap();

        Self::new(&v_shader, &f_shader)
    }

    pub fn from_shaders(shaders: &[&Shader]) -> Result<Program, String> {
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
            println!("===== Compilation Error! ===== ");
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
        Program::from_shaders(&[v_shader, f_shader]).unwrap()
    }

    pub fn scoped_binder(&self) -> ScopedBinder {
        ScopedBinder::new(self.id)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        println!("Destroying program {}", self.id);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
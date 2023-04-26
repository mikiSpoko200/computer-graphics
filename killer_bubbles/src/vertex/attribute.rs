use std::fmt::Debug;
use gl::types::GLenum;
use crate::gl::{BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT, INT, UNSIGNED_INT, FLOAT, DOUBLE};

/// primitive type compatible with opengl on the ABI layer
pub unsafe trait Primitive: Copy + Debug {
    const GL_TYPE: GLenum;
}

unsafe impl Primitive for u8  { const GL_TYPE: GLenum = UNSIGNED_BYTE; }
unsafe impl Primitive for u16 { const GL_TYPE: GLenum = UNSIGNED_SHORT; }
unsafe impl Primitive for u32 { const GL_TYPE: GLenum = UNSIGNED_INT; }
unsafe impl Primitive for i8  { const GL_TYPE: GLenum = BYTE; }
unsafe impl Primitive for i16 { const GL_TYPE: GLenum = SHORT; }
unsafe impl Primitive for i32 { const GL_TYPE: GLenum = INT; }
unsafe impl Primitive for f32 { const GL_TYPE: GLenum = FLOAT; }
unsafe impl Primitive for f64 { const GL_TYPE: GLenum = DOUBLE; }

/// Prototype for VertexArity enum that checks the arity of the attribute
#[cfg(any())]
mod _vertex_arity {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum VertexArity {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Nine = 9,
        Sixteen = 16,
    }

    impl VertexArity {
        pub fn new<const N: usize>() -> Self {
            match N {
                1 => Self::One,
                2 => Self::Two,
                3 => Self::Three,
                _ => todo!()
            }
        }
    }
}

/// Universal format specifier for vertex attributes
pub struct VertexFormat {
    pub arity: usize,
    pub size: usize,
    pub gl_type: GLenum,
}

/// todo: rename
impl VertexFormat {
    pub const fn from_type<P: Primitive, const N: usize>() -> Self {
        Self {
            arity: N,
            size: std::mem::size_of::<P>(),
            gl_type: gl_type::<P>()
        }
    }

    pub unsafe fn new(arity: usize, size: usize, gl_type: GLenum) -> Self {
        Self {
            arity,
            size,
            gl_type,
        }
    }
}

/// todo: rename?
pub trait Attribute {
    fn format(&self) -> VertexFormat;
}

/// Convenience function for accessing the Primitive's opengl type code.
pub fn gl_type<P: Primitive>() -> gl::GLenum {
    P::GL_TYPE
}

impl<T, P, const N: usize> Attribute for T
where
    T: AsRef<[P; N]>,
    P: Primitive
{
    fn format(&self) -> VertexFormat {
        VertexFormat::from_type::<P, N>()
    }
}

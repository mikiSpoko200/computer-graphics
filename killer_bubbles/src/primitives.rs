use std::fmt::Debug;
use gl::types::GLenum;
use crate::gl::{BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT, INT, UNSIGNED_INT, FLOAT, DOUBLE};

// note: There are two kinds of primitive types - primitive types of glsl,
//  and primitive types on the CPU side.
//  GLSL does not understand RGB_10_A_2 type for example. This type along others exists only for
//  storage convenience. Upon transmission to the GPU they are changed to corresponding GLSL types.

/// primitive type compatible with opengl on the ABI layer
/// Known size ang internal layout
/// note: alignment is platform specific
pub unsafe trait Primitive: Copy + Debug + Sized {
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

use std::fmt::Debug;
use gl::types::GLenum;
use crate::gl::{BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT, INT, UNSIGNED_INT, FLOAT, DOUBLE};

pub trait Primitive: Copy + Debug {
    const GL_TYPE: GLenum;
}

impl Primitive for u8 { const GL_TYPE: GLenum = UNSIGNED_BYTE; }
impl Primitive for u16 { const GL_TYPE: GLenum = UNSIGNED_SHORT; }
impl Primitive for u32 { const GL_TYPE: GLenum = UNSIGNED_INT; }
impl Primitive for i8 { const GL_TYPE: GLenum = BYTE; }
impl Primitive for i16 { const GL_TYPE: GLenum = SHORT; }
impl Primitive for i32 { const GL_TYPE: GLenum = INT; }
impl Primitive for f32 { const GL_TYPE: GLenum = FLOAT; }
impl Primitive for f64 { const GL_TYPE: GLenum = DOUBLE; }

impl<P: Primitive> From<P> for VertexAttribute<P, 1> {
    fn from(data: P) -> Self { Self { data: [data] } }
}

impl<P: Primitive> From<(P, P)> for VertexAttribute<P, 2> {
    fn from(data: (P, P)) -> Self { Self { data: [data.0, data.1] } }
}

impl<P: Primitive> From<(P, P, P)> for VertexAttribute<P, 3> {
    fn from(data: (P, P, P)) -> Self { Self { data: [data.0, data.1, data.2] } }
}

impl<P: Primitive> From<(P, P, P, P)> for VertexAttribute<P, 4> {
    fn from(data: (P, P, P, P)) -> Self { Self { data: [data.0, data.1, data.2, data.3] } }
}

impl<P: Primitive> From<[P; 2]> for VertexAttribute<P, 2> {
    fn from(data: [P; 2]) -> Self { Self { data } }
}

impl<P: Primitive> From<[P; 3]> for VertexAttribute<P, 3> {
    fn from(data: [P; 3]) -> Self { Self { data } }
}

impl<P: Primitive> From<[P; 4]> for VertexAttribute<P, 4> {
    fn from(data: [P; 4]) -> Self { Self { data } }
}

impl<P: Primitive> From<[P; 9]> for VertexAttribute<P, 9> {
    fn from(data: [P; 9]) -> Self { Self { data } }
}

impl<P: Primitive> From<[P; 16]> for VertexAttribute<P, 16> {
    fn from(data: [P; 16]) -> Self { Self { data } }
}

impl<P: Primitive> From<[[P; 2]; 2]> for VertexAttribute<P, 4> {
    fn from(data: [[P; 2]; 2]) -> Self {
        let data = [data[0][0], data[0][1], data[1][0], data[1][1]];
        Self { data }
    }
}

impl<P: Primitive> From<[[P; 3]; 3]> for VertexAttribute<P, 9> {
    fn from(data: [[P; 3]; 3]) -> Self {
        let data = [
            data[0][0], data[0][1], data[0][2],
            data[1][0], data[1][1], data[1][2],
            data[2][0], data[2][1], data[2][2],
        ];
        Self { data }
    }
}

impl<P: Primitive> From<[[P; 4]; 4]> for VertexAttribute<P, 16> {
    fn from(data: [[P; 4]; 4]) -> Self {
        let data = [
            data[0][0], data[0][1], data[0][2], data[0][3],
            data[1][0], data[1][1], data[1][2], data[1][3],
            data[2][0], data[2][1], data[2][2], data[2][3],
            data[3][0], data[3][1], data[3][2], data[3][3],
        ];
        Self { data }
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttribute<P: Primitive, const COMPONENT_COUNT: usize> {
    data: [P; COMPONENT_COUNT],
}

impl<P: Primitive, const COMPONENT_COUNT: usize> VertexAttribute<P, COMPONENT_COUNT> {
    pub fn attribute_type() -> AttributeType {
        AttributeType::from_attribute_params::<P, COMPONENT_COUNT>()
    }
}

impl<P: Primitive, const COMPONENT_COUNT: usize> AsRef<[P]> for VertexAttribute<P, COMPONENT_COUNT> {
    fn as_ref(&self) -> &[P] {
        self.data.as_ref()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttributeType {
    component_count: usize,
    byte_size: usize,
    gl_type_enum: GLenum,
}

impl AttributeType {
    pub fn component_count(&self) -> usize {
        self.component_count
    }

    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    pub fn gl_type(&self) -> GLenum {
        self.gl_type_enum
    }

    pub const fn from_attribute_params<P: Primitive, const COMPONENT_COUNT: usize>() -> Self {
        Self {
            component_count: COMPONENT_COUNT,
            byte_size: std::mem::size_of::<P>(),
            gl_type_enum: P::GL_TYPE
        }
    }
}

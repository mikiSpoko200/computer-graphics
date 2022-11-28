use std::fmt::Debug;
use gl::types::{GLenum, GLuint};
use crate::gl_assert_no_err;

pub trait GlBufferTargetProvider {
    const TARGET: GLuint;

    fn get_buffer_target() -> GLuint { Self::TARGET }
}

pub trait IndexBuffer {
    fn upload(&self);

    fn id(&self) -> GLuint;

    fn vertex_count(&self) -> usize;

    fn index_type(&self) -> IndexType;

    fn scoped_binder(&self) -> ScopedBinder {
        ScopedBinder::new(self.id())
    }
}

impl<B: IndexBuffer> GlBufferTargetProvider for B {
    const TARGET: GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

/// Marker that type can be used as opengl index for element buffer.
/// Associated constant INDEX_TYPE, maps type to appropriate IndexType.
pub trait IndexingPrimitive {
    const INDEX_TYPE: IndexType;
}

impl IndexingPrimitive for u8 { const INDEX_TYPE: IndexType = IndexType::U8; }
impl IndexingPrimitive for u16 { const INDEX_TYPE: IndexType = IndexType::U16; }
impl IndexingPrimitive for u32 { const INDEX_TYPE: IndexType = IndexType::U32; }

/// Representation of types that can be used as indices for indexed drawing in opengl.
#[derive(Debug, Copy, Clone)]
pub enum IndexType {
    U8,
    U16,
    U32
}

impl IndexType {
    pub fn get_gl_type(&self) -> GLenum {
        match *self {
            Self::U8 => gl::UNSIGNED_BYTE,
            Self::U16 => gl::UNSIGNED_SHORT,
            Self::U32 => gl::UNSIGNED_INT,
        }
    }

    pub fn from_type<IP: IndexingPrimitive>() -> Self {
        IP::INDEX_TYPE
    }
}

pub type IndexingMode<P> = Option<P>;

pub struct IndexBufferObject<P: IndexingPrimitive = u32> {
    id: GLuint,
    indices: Box<[P]>,
}

impl<P: IndexingPrimitive> IndexBuffer for IndexBufferObject<P> {
    fn upload(&self) {
        gl_assert_no_err!();
        unsafe {
            gl::BufferData(
                <Self as GlBufferTargetProvider>::TARGET,
                (self.vertex_count() * std::mem::size_of::<P>()) as _,
                self.indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW
            );
        }
        gl_assert_no_err!();
    }

    fn id(&self) -> GLuint {
        self.id
    }

    fn vertex_count(&self) -> usize {
        self.indices.len()
    }

    fn index_type(&self) -> IndexType {
        P::INDEX_TYPE
    }
}

impl<P: IndexingPrimitive> IndexBufferObject<P> {
    pub fn create(indices: Box<[P]>) -> Self {
        gl_assert_no_err!();
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        gl_assert_no_err!();
        Self { id, indices }
    }
}

impl<P: IndexingPrimitive> Drop for IndexBufferObject<P> {
    fn drop(&mut self) {
        gl_assert_no_err!();
        unsafe { gl::DeleteBuffers(1, &self.id); }
        gl_assert_no_err!();
    }
}


pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(id : GLuint) -> Self {
        gl_assert_no_err!();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
        }
        gl_assert_no_err!();
        Self(id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        gl_assert_no_err!();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0);
        }
        gl_assert_no_err!();
    }
}

use super::{Primitive, VertexAttribute, AttributeType};
use gl;
use gl::types::GLuint;

use crate::{gl_assert, gl_assert_no_err};
use crate::index_buffer::{GlBufferTargetProvider};

#[macro_export]
macro_rules! attributes {
    () => {
        []
    };
    ($($x:expr),+ $(,)?) => {
        $crate::vertex::BufferObject::create(
            vec!($( $crate::vertex::VertexAttribute::from($x)), +).into_boxed_slice()
        )
    };
}

/// Trait that represents an owner of vertex::Attributes
pub trait Buffer {
    fn upload(&self);

    fn id(&self) -> GLuint;

    fn attribute_type(&self) -> AttributeType;

    fn vertex_count(&self) -> usize;

    fn scoped_binder(&self) -> ScopedBinder {
        gl_assert_no_err!();
        ScopedBinder::new(self.id())
    }
}

//region BufferObject
/// Buffer object for packed vertex attributes
#[derive(Debug)]
pub struct BufferObject<P: Primitive, const N: usize> {
    id: GLuint,
    buffer: Box<[VertexAttribute<P, N>]>,
}

impl<P: Primitive, const N: usize> BufferObject<P, N> {
    pub fn create(buffer: Box<[VertexAttribute<P, N>]>) -> Self {
        let mut id = 0;
        unsafe {
            gl_assert!(gl::CreateBuffers(1, &mut id));
        }
        Self { id, buffer }
    }
}

impl<P: Primitive, const N: usize> From<&[VertexAttribute<P, N>]> for BufferObject<P, N> {
    fn from(data: &[VertexAttribute<P, N>]) -> Self {
        let attrs = data.to_owned().into_boxed_slice();
        Self::create(attrs)
    }
}

impl<P: Primitive, const N: usize> GlBufferTargetProvider for BufferObject<P, N> {
    const TARGET: GLuint = gl::ARRAY_BUFFER;
}

impl<P: Primitive, const N: usize> Buffer for BufferObject<P, N> {
    fn upload(&self) {
        gl_assert_no_err!();
        let byte_count = self.vertex_count() * std::mem::size_of::<VertexAttribute<P, N>>();
        let raw_ptr = self.buffer.as_ref().as_ptr() as *const std::ffi::c_void;
        unsafe {
            gl::BufferData(<Self as GlBufferTargetProvider>::TARGET, byte_count as _, raw_ptr, gl::STATIC_DRAW);
        }
        gl_assert_no_err!();
    }

    fn id(&self) -> GLuint {
        self.id
    }

    fn attribute_type(&self) -> AttributeType {
        VertexAttribute::<P, N>::attribute_type()
    }

    fn vertex_count(&self) -> usize {
        self.buffer.len()
    }
}

impl<P: Primitive, const N: usize> Drop for BufferObject<P, N> {
    fn drop(&mut self) {
        log::debug!("Deleting buffer object {}", self.id);
        unsafe {
            gl::DeleteBuffers(1, &self.id)
        }
    }
}
//endregion

//region ScopedBinder
pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(buffer_id: GLuint) -> Self {
        log::debug!("Binding buffer object {}", buffer_id);
        gl_assert_no_err!();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
        }
        gl_assert_no_err!();
        Self(buffer_id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        log::debug!("Unbinding buffer object {}", self.0);
        gl_assert_no_err!();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        gl_assert_no_err!();
    }
}
//endregion

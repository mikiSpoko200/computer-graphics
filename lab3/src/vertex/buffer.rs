use super::{GlPrimitive, AttributeArray};
use gl;
use gl::types::GLuint;
use crate::vertex::attribute::AttributeType;

use crate::gl_assert;

/// Trait that represents an owner of vertex::Attributes
pub trait Buffer {
    fn upload(&self);

    fn id(&self) -> GLuint;

    fn attr_type(&self) -> AttributeType;

    fn scoped_binder(&self) -> ScopedBinder {
        ScopedBinder::new(self.id())
    }
}

//region BufferObject
///
#[derive(Debug)]
pub struct BufferObject<P: GlPrimitive> {
    id: GLuint,
    buffer: Vec<P>,
    attr_type: AttributeType
}

impl<P: GlPrimitive> BufferObject<P> {
    pub fn create<B>(attrs: &B, attr_type: AttributeType) -> Self
    where
        B: AsRef<[AttributeArray<P>]>
    {
        let mut local = Vec::new();
        for attr in attrs.as_ref() {
            local.extend(attr.as_ref())
        }

        for elem in &local {
            println!("{:?}", elem);
        }

        let mut id = 0;
        unsafe {
            gl_assert!(gl::CreateBuffers(1, &mut id));
        }
        Self { id, buffer: local, attr_type }
    }
}

impl<P: GlPrimitive> Buffer for BufferObject<P> {
    fn upload(&self) {
        assert!(dbg!(unsafe { gl::GetError() }) == gl::NO_ERROR);
        unsafe {
            gl_assert!(
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.buffer.len() * std::mem::size_of::<P>()) as _,
                    self.buffer.as_ptr() as *const std::ffi::c_void,
                    gl::STATIC_DRAW
                )
            );
        }
    }

    fn id(&self) -> GLuint {
        self.id
    }

    fn attr_type(&self) -> AttributeType {
        self.attr_type
    }
}
//endregion

//region ScopedBinder
pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(buffer_id: GLuint) -> Self {
        println!("Binding buffer object {buffer_id}");
        assert!(unsafe { gl::GetError() } == gl::NO_ERROR);
        unsafe { gl_assert!(gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id)); }
        Self(buffer_id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        println!("Unbinding buffer object {}", self.0);
        unsafe { gl::BindBuffer(gl::VERTEX_ARRAY, 0); }
    }
}
//endregion

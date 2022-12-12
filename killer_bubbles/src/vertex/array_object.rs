use gl::types::GLuint;
use crate::vertex::AttributeType;

use crate::gl_assert_no_err;

pub struct ArrayObject {
    id: GLuint
}

impl ArrayObject {
    pub fn create() -> Self {
        let mut id = 0;
        gl_assert_no_err!();
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        gl_assert_no_err!();
        Self { id }
    }

    pub fn scoped_binder(&self) -> ScopedBinder {
        ScopedBinder::new(self.id)
    }

    pub fn set_attrib_divisor(&self, layout: usize, divisor: usize) {
        gl_assert_no_err!();
        unsafe {
            gl::VertexAttribDivisor(layout as _, divisor as _);
        }
        gl_assert_no_err!();
    }

    pub fn set_vertex_attrib_pointer(&self, layout: usize, attr: &AttributeType) {
        gl_assert_no_err!();
        unsafe {
            gl::EnableVertexAttribArray(layout as _);
        }
        gl_assert_no_err!();
        unsafe {
            gl::VertexAttribPointer(
                layout as _,
                attr.component_count() as _,
                attr.gl_type(),
                gl::FALSE,
                0,
                std::ptr::null(),
            );
        }
        gl_assert_no_err!();
    }
}

pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(vao_id: GLuint) -> Self {
        gl_assert_no_err!();
        unsafe { gl::BindVertexArray(vao_id); }
        gl_assert_no_err!();
        Self(vao_id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        gl_assert_no_err!();
        unsafe { gl::BindVertexArray(0); }
        gl_assert_no_err!();
    }
}
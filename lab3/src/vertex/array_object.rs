use gl::types::GLuint;
use std::collections::HashSet;
use crate::vertex::attribute::AttributeType;

pub struct ArrayObject {
    enabled_attrs: HashSet<GLuint>,
    id: GLuint
}

impl ArrayObject {
    pub fn create() -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        Self { id, enabled_attrs: HashSet::new() }
    }

    pub fn scoped_binder(&self) -> ScopedBinder {
        ScopedBinder::new(self.id)
    }

    pub fn set_vertex_attrib_pointer(&mut self, layout: GLuint, attr: &AttributeType, _binder: &ScopedBinder) {
        println!("Setting attribute pointer");
        if !self.enabled_attrs.contains(&layout) {
            self.enabled_attrs.insert(layout);
            unsafe {
                gl::EnableVertexAttribArray(layout);
            }
        }

        unsafe {
            gl::VertexAttribPointer(
                layout,
                attr.component_count() as _,
                attr.component_type(),
                0,
                0,
                std::ptr::null(),
            )
        }
    }
}

pub struct ScopedBinder(GLuint);

impl ScopedBinder {
    pub fn new(vao_id: GLuint) -> Self {
        println!("Binding vao {vao_id}");
        unsafe { gl::BindVertexArray(vao_id); }
        Self(vao_id)
    }
}

impl Drop for ScopedBinder {
    fn drop(&mut self) {
        println!("Unbinding vao {}", self.0);
        unsafe { gl::BindVertexArray(0); }
    }
}
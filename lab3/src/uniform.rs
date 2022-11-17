extern crate proc_macro;
use proc_macro::TokenStream;
use gl::types::{GLint, GLuint};

pub unsafe trait Uniform {
    fn bind(&self, location: GLint);
}

unsafe impl Uniform for f32 {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform1f(location, *self); }
    }
}

unsafe impl Uniform for (f32, f32) {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform2f(location, *self.0, *self.1); }
    }
}

unsafe impl Uniform for (f32, f32, f32) {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform3f(location, *self.0, *self.1, *self.2); }
    }
}

unsafe impl Uniform for (f32, f32, f32, f32) {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform4f(location, *self.0, *self.1, *self.2, *self.3); }
    }
}

unsafe impl Uniform for [f32; 2] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform2f(location, self[0], self[1]); }
    }
}

unsafe impl Uniform for [f32; 3] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform3f(location, self[0], self[1], self[2]); }
    }
}

unsafe impl Uniform for [f32; 4] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform4f(location, self[0], self[1], self[2], self[3]); }
    }
}

unsafe impl Uniform for [[f32; 3]; 3] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform3fv(location, 1, self.as_ptr().as_ptr()); }
    }
}

unsafe impl Uniform for [[f32; 4]; 4] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform4fv(location, 1, self.as_ptr().as_ptr()); }
    }
}

unsafe impl Uniform for [f32; 4] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform2fv(location, 1, self.as_ptr()); }
    }
}

unsafe impl Uniform for [f32; 9] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform3fv(location, 1, self.as_ptr()); }
    }
}

unsafe impl Uniform for [f32; 16] {
    fn bind(&self, location: GLint) {
        unsafe { crate::gl::Uniform4fv(location, 1, self.as_ptr()); }
    }
}

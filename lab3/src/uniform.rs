use gl::types::GLint;
use crate::{gl_assert, gl_assert_no_err};

pub unsafe trait Uniform {
    fn bind(&self, location: GLint);
}

unsafe impl Uniform for f32 {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform1f(location, *self); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for (f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform2f(location, self.0, self.1); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for (f32, f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform3f(location, self.0, self.1, self.2); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for (f32, f32, f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform4f(location, self.0, self.1, self.2, self.3); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for [f32; 2] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform2f(location, self[0], self[1]);
        gl_assert_no_err!();
        }
    }
}

unsafe impl Uniform for [f32; 3] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform3f(location, self[0], self[1], self[2]); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for [f32; 4] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform4f(location, self[0], self[1], self[2], self[3]); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for [[f32; 3]; 3] {
    fn bind(&self, location: GLint) {
        let &[[a, b, c], [d, e, f], [g, h, i]] = self;
        let data = [a, b, c, d, e, f, g, h, i];
        data.bind(location);
    }
}

unsafe impl Uniform for [[f32; 4]; 4] {
    fn bind(&self, location: GLint) {
        let &[[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self;
        let data = [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p];
        data.bind(location);
    }
}

unsafe impl Uniform for [f32; 9] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_ptr()); }
        gl_assert_no_err!();
    }
}

unsafe impl Uniform for [f32; 16] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr()); }
        gl_assert_no_err!();
    }
}

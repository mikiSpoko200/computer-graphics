use std::fmt::{Debug, Display, Formatter};
use gl::types::GLint;
use crate::{gl_assert_no_err};

pub type NamedUniform = (&'static str, Box<dyn TypedUniform>);

pub enum UniformType {
    Float,
    Vec2,
    Vec3,
    Mat4,
}

pub fn to_owned<U, T, I>(uniforms: I) -> impl Iterator<Item=(&'static str, Box<dyn TypedUniform>)>
where
    U: 'static + TypedUniform + Clone,
    T: AsRef<U>,
    I: IntoIterator<Item=(&'static str, T)>
{
    uniforms.into_iter()
        .map(|(ident, uniform)| {
            (ident, Box::new(uniform.as_ref().clone()) as _)
        })
}

pub trait UniformTypeProvider {
    fn uniform_type(&self) -> UniformType;
}

impl Display for UniformType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r#type = match self {
            UniformType::Float => "float",
            UniformType::Vec3 => "vec3",
            UniformType::Vec2 => "vec2",
            UniformType::Mat4 => "mat4"
        };
        write!(f, "uniform {}", r#type)?;
        Ok(())
    }
}

pub trait Uniform: Debug {
    fn bind(&self, location: GLint);
}

pub trait TypedUniform: Uniform + UniformTypeProvider { }

impl<TU: Uniform + UniformTypeProvider> TypedUniform for TU {}

impl Uniform for f32 {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform1f(location, *self); }
        gl_assert_no_err!();
    }
}

impl UniformTypeProvider for f32 {
    fn uniform_type(&self) -> UniformType {
        UniformType::Float
    }
}

impl UniformTypeProvider for [f32; 3] {
    fn uniform_type(&self) -> UniformType {
        UniformType::Vec3
    }
}

impl UniformTypeProvider for (f32, f32, f32) {
    fn uniform_type(&self) -> UniformType {
        UniformType::Vec3
    }
}

impl UniformTypeProvider for [f32; 16] {
    fn uniform_type(&self) -> UniformType {
        UniformType::Mat4
    }
}
impl UniformTypeProvider for [[f32; 4]; 4] {
    fn uniform_type(&self) -> UniformType {
        UniformType::Mat4
    }
}

impl Uniform for (f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform2f(location, self.0, self.1); }
        gl_assert_no_err!();
    }
}

impl Uniform for (f32, f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform3f(location, self.0, self.1, self.2); }
        gl_assert_no_err!();
    }
}

impl Uniform for (f32, f32, f32, f32) {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform4f(location, self.0, self.1, self.2, self.3); }
        gl_assert_no_err!();
    }
}

impl Uniform for [f32; 2] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform2f(location, self[0], self[1]);
        gl_assert_no_err!();
        }
    }
}

impl Uniform for [f32; 3] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform3f(location, self[0], self[1], self[2]); }
        gl_assert_no_err!();
    }
}

impl Uniform for [f32; 4] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::Uniform4f(location, self[0], self[1], self[2], self[3]); }
        gl_assert_no_err!();
    }
}

impl Uniform for [[f32; 3]; 3] {
    fn bind(&self, location: GLint) {
        let &[[a, b, c], [d, e, f], [g, h, i]] = self;
        let data = [a, b, c, d, e, f, g, h, i];
        data.bind(location);
    }
}

impl Uniform for [[f32; 4]; 4] {
    fn bind(&self, location: GLint) {
        let &[[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self;
        let data = [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p];
        data.bind(location);
    }
}

impl Uniform for [f32; 9] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_ptr()); }
        gl_assert_no_err!();
    }
}

impl Uniform for [f32; 16] {
    fn bind(&self, location: GLint) {
        gl_assert_no_err!();
        unsafe { crate::gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr()); }
        gl_assert_no_err!();
    }
}

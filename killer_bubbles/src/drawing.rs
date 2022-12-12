use gl::types::GLenum;
use crate::gl_assert_no_err;
use crate::index_buffer::IndexType;

/// Opengl primitive draw mode enumeration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DrawMode {
    Triangles,
    TriangleStrip,
    TriangleFan,
    Points,
    Lines,
    LineStrip,
    LineLoop,
}

impl DrawMode {
    /// Get opengl defined numerical value of given variant
    pub fn get(&self) -> GLenum {
        match *self {
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Points => gl::POINTS,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
        }
    }
}

pub fn draw_arrays(draw_mode: &DrawMode, vertex_count: usize) {
    gl_assert_no_err!();
    unsafe {
        gl::DrawArrays(draw_mode.get(), 0, vertex_count as _);
    }
    gl_assert_no_err!();
}

pub fn draw_indexed(draw_mode: &DrawMode, vertex_count: usize, index_type: &IndexType) {
    gl_assert_no_err!();
    unsafe {
        gl::DrawElements(
            draw_mode.get(),
            vertex_count as _,
            index_type.get_gl_type(),
            std::ptr::null()
        );
    }
    gl_assert_no_err!();
}

pub mod instanced {
    use crate::gl_assert_no_err;
    use super::DrawMode;
    use crate::index_buffer::IndexType;

    pub fn draw_arrays(draw_mode: &DrawMode, vertex_count: usize, instance_count: usize) {
        gl_assert_no_err!();
        unsafe {
            gl::DrawArraysInstanced(
                draw_mode.get(),
                0,
                3 as _, // vertex_count as
                instance_count as _,
            );
        }
        gl_assert_no_err!();
    }

    pub fn draw_indexed(draw_mode: &DrawMode, vertex_count: usize, index_type: &IndexType, instance_count: usize) {
        gl_assert_no_err!();
        unsafe {
            gl::DrawElementsInstanced(
                draw_mode.get(),
                vertex_count as _,
                index_type.get_gl_type(),
                std::ptr::null(),
                instance_count as _
            );
        }
        gl_assert_no_err!();
    }
}
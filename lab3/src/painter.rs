use crate::{binder, vertex, program, uniform, drawing, index_buffer};

use crate::gl_assert_no_err;
use uniform::Uniform;
use program::Program;
use drawing::DrawMode;
use index_buffer::{IndexBuffer, IndexingMode, IndexType, IndexBufferObject};
use vertex::{VertexAttribute, BufferObject};
use binder::Binder;


pub struct Painter<I: IndexBuffer> {
    binder: Binder<I>,
    draw_mode: DrawMode,
    instance_count: Option<usize>,
}

impl<I: IndexBuffer> Painter<I> {
    pub fn new(binder: Binder<I>, draw_mode: DrawMode) -> Self {
        Self {
            binder,
            draw_mode,
            instance_count: None
        }
    }

    pub fn binder(&self) -> &Binder<I> {
        &self.binder
    }

    pub fn binder_mut(&mut self) -> &mut Binder<I> { &mut self.binder }

    pub fn instanced(mut self, instance_count: usize) -> Self {
        self.instance_count = Some(instance_count);
        self
    }

    pub fn update_draw_mode(&mut self, new: DrawMode) {
        self.draw_mode = new;
    }

    pub fn draw(&self) {
        let _draw_scoped_binder = self.binder.draw_binder();
        match (self.instance_count, self.binder.index_type()) {
            (Some(instance_count), Some(ref index_type)) => {
                drawing::instanced::draw_indexed(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    index_type,
                    instance_count
                );
            },
            (Some(instance_count), None) => {
                drawing::instanced::draw_arrays(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    instance_count
                );
            },
            (None, Some(ref index_type)) => {
                drawing::draw_indexed(
                    &self.draw_mode,
                    self.binder.vertex_count(),
                    index_type
                );
            },
            (None, None) => {
                drawing::draw_arrays(&self.draw_mode, self.binder.vertex_count());
            }
        }
    }
}

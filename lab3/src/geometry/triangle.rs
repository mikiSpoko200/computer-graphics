use crate::{binder, vertex, program, index_buffer, uniform};

use program::Program;
use index_buffer::IndexBufferObject;
use vertex::{VertexAttribute, BufferObject};
use binder::Binder;
use uniform::NamedUniform;

use nalgebra_glm as glm;


pub fn template_triangle(a: f32) -> [glm::Vec3; 3] {
    let radius = a / f32::sqrt(3.0);
    [
        glm::vec3(0.0, radius, 0.0),
        glm::vec3(-a / 2.0, -radius / 2.0, 0.0),
        glm::vec3(a / 2.0, -radius / 2.0, 0.0)
    ]
}

pub fn labyrinth(uniforms: impl Iterator<Item=NamedUniform>, n: usize) -> Binder<IndexBufferObject> {
    let scale = 1.0 / n as f32;
    let tail_center_offset = glm::vec3(1f32, 1f32, 1f32) / (2.0 * n as f32);

    let scaled_model = template_triangle(2.0).into_iter().map(|position| {
        position * scale
    });
    let mut positions: Vec<VertexAttribute<f32, 3>> = Vec::new();

    for xi in 0..n {
        for yi in 0..n {
            for zi in 0..n {
                let corner_offset = glm::vec3(xi as _, yi as _, zi as _) / n as f32;
                let center_offset = corner_offset + tail_center_offset;
                let center_offset_ndc = 2.0 * center_offset - glm::vec3(1.0, 1.0, 1.0);
                for position in scaled_model.clone() {
                    let arr = (position + center_offset_ndc).as_ref().clone();
                    positions.push(VertexAttribute::from(arr));
                }
            }
        }
    }

    let buffer_object = BufferObject::create(positions.into_boxed_slice());

    let program = Program::from_file(
        "shaders/labyrinth_v.glsl".as_ref(),
        "shaders/labyrinth_f.glsl".as_ref()
    );

    let mut binder = Binder::new(
        vec!(Box::new(buffer_object)),
        None,
        program,
        uniforms
    );
    binder.upload();
    binder
}

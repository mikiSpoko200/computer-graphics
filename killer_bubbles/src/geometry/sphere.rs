use crate::{binder, vertex, program, index_buffer, uniform};

use program::Program;
use index_buffer::IndexBufferObject;
use vertex::{VertexAttribute, BufferObject};
use binder::Binder;
use uniform::NamedUniform;

use nalgebra_glm as glm;


pub fn sp(radius: f32, poly_count: usize) -> (Box<[VertexAttribute<f32, 3>]>, Box<[VertexAttribute<f32, 3>]>, Box<[u16]>) {
    use std::f32::consts::PI;

    let mut vertices = Vec::new();
    let mut normals = Vec::new();

    let sector_angle_offset = 2.0 * PI / poly_count as f32;
    let stack_angle_offset = PI / poly_count as f32;

    for stack_index in 0..=poly_count {
        let stack_angle = PI / 2.0 - stack_index as f32 * stack_angle_offset;
        let xy = radius * f32::cos(stack_angle);
        let  z = radius * f32::sin(stack_angle);

        for sector_index in 0..=poly_count {
            let sector_angle = sector_index as f32 * sector_angle_offset;
            let x = xy * f32::cos(sector_angle);
            let y = xy * f32::sin(sector_angle);
            let point = glm::Vec3::new(x, y, z);

            vertices.push(VertexAttribute::from(*point.as_ref()));
            normals.push(VertexAttribute::from(*(point / radius).as_ref()));
        }
    }

    let mut indices = Vec::new();
    for stack_index in 0..poly_count {
        let k1 = (stack_index * (poly_count + 1)) as u16;
        let k2 = (k1 as usize + poly_count + 1) as u16;

        for (k1, k2) in (k1..).zip(k2..).take(poly_count) {
            if stack_index != 0 {
                indices.push(k1);
                indices.push(k2);
                indices.push(k1 + 1);
            }
            if stack_index != poly_count - 1 {
                indices.push(k1 + 1);
                indices.push(k2);
                indices.push(k2 + 1);
            }
        }
    }

    (vertices.into_boxed_slice(), normals.into_boxed_slice(), indices.into_boxed_slice())
}

pub fn sphere(uniforms: impl Iterator<Item=NamedUniform>) -> Binder<IndexBufferObject<u16>> {
    let (vertices, normals, indices) = sp(1.0, 25);

    let positions = Box::new(BufferObject::create(vertices));
    let normals = Box::new(BufferObject::create(normals));
    let index_buf = IndexBufferObject::create(indices);

    let program = Program::from_file(
        "shaders/sphere_v.glsl".as_ref(),
        "shaders/sphere_f.glsl".as_ref()
    );

    let mut binder = Binder::new(
        vec!(positions, normals),
        Some(index_buf),
        program,
        uniforms
    );
    binder.upload();
    binder
}

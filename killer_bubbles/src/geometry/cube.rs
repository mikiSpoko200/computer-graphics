use crate::{binder, vertex, program, index_buffer, uniform, attributes};

use program::Program;
use index_buffer::IndexBufferObject;
use binder::Binder;
use uniform::NamedUniform;


pub fn cube(uniforms: impl Iterator<Item=NamedUniform>) -> Binder<IndexBufferObject<u8>> {
    let vertices = attributes!(
         (-1.0, -1.0, -1.0),    // 000
         (-1.0, -1.0,  1.0),    // 001
         (-1.0,  1.0, -1.0),    // 010
         (-1.0,  1.0,  1.0),    // 011
         ( 1.0, -1.0, -1.0),    // 100
         ( 1.0, -1.0,  1.0),    // 101
         ( 1.0,  1.0, -1.0),    // 110
         ( 1.0,  1.0,  1.0f32), // 111
    );

    let colors = attributes!(
        (0.1, 0.1, 0.1),
        (0.1, 0.1, 0.1),
        (0.0, 0.4, 0.73),
        (0.0, 0.4, 0.73),
        (0.1, 0.1, 0.1),
        (0.1, 0.1, 0.1),
        (0.0, 0.4, 0.73),
        (0.0, 0.4, 0.73f32),
    );

    let indices = Box::new([
        0, 1, 5,
        0, 5, 4,
        0, 1, 3,
        0, 3, 2,
        0, 4, 6,
        0, 6, 2,
        1, 5, 7,
        1, 7, 3,
        5, 6, 4,
        5, 7, 6,
        7, 3, 2,
        2, 7, 6
    ]);

    let index_buf = IndexBufferObject::create(indices);
    let program = Program::from_file(
        "shaders/cube_v.glsl".as_ref(),
        "shaders/cube_f.glsl".as_ref(),
    );

    let mut binder = Binder::new(
        vec!(Box::new(vertices), Box::new(colors)),
        Some(index_buf),
        program,
        uniforms
    );
    binder.upload();
    binder
}

use alloc::rc::Rc;
use crate::{attributes, Binder};
use crate::index_buffer::{IndexBufferObject};
use crate::program::Program;


pub fn cube() -> Binder<IndexBufferObject<u8>> {
    let vertices = attributes!(
         (-1.0, -1.0, -1.0), // 0
         (-1.0, -1.0,  1.0), // 1
         (-1.0,  1.0, -1.0), // 2
         (-1.0,  1.0,  1.0), // 3
         ( 1.0, -1.0, -1.0), // 4
         ( 1.0, -1.0,  1.0), // 5
         ( 1.0,  1.0, -1.0), // 6
         ( 1.0,  1.0,  1.0f32), // 7
    );

    let colors = attributes!(
        (1.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (1.0, 0.0, 1.0),
        (0.0, 1.0, 1.0),
        (1.0, 1.0, 1.0),
        (0.0, 1.0, 1.0f32),
    );

    let indices = Box::new([
        3, 7, 1,
        5, 4, 7,
        6, 3, 2,
        1, 0, 4,
        2, 6
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
        vec!()
    );
    binder.upload();
    binder
}

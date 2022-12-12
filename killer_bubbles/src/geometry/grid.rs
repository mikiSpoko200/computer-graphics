use crate::{ComponentProvider, Transform};

use crate::binder::Binder;
use crate::index_buffer::IndexBufferObject;

pub struct Grid {
    transform: Transform,
    binder: Binder<IndexBufferObject>,
}

impl ComponentProvider<Transform> for Grid {
    fn component(&self) -> &Transform {
        &self.transform
    }

    fn component_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
//
// impl Grid {
//     pub fn new(line_count: usize, line_sep: f32, transform: Transform, color: Vec3) -> Self {
//
//         let positions = vecc![VertexAttribute::from((x, y)); for x in 0..line_count; for y in 0..lines_count];
//         let buffer = BufferObject::create(positions.into());
//
//         let program = Program::from_file(
//             "shaders/grid_v.glsl".as_ref(),
//             "shaders/grid_f.glsl".as_ref()
//         );
//
//         let binder = Binder::new(
//             vec!(Box::new(buffer)),
//             None,
//             program,
//
//         )
//
//         Self { line_sep, transform, color }
//     }
//
//     pub fn draw(&self) {
//
//     }
// }
//
// impl Default for Grid {
//     fn default() -> Self {
//         Self::new(
//             0.1,
//             Transform::default(),
//             Scene::DARK_GRAY,
//         )
//     }
// }
use crate::binder::Binder;
use crate::{Direction, Directions};
use crate::index_buffer::IndexBufferObject;
use crate::program::Program;
use crate::uniform::NamedUniform;

pub fn axis(uniforms: impl Iterator<Item=NamedUniform>, direction: Direction) -> Binder<IndexBufferObject<u16>> {
    let program = Program::from_file(
        "shaders/line_v.glsl".as_ref(),
        "shaders/line_f.glsl".as_ref()
    );
    let point = match direction {
        Direction::Front => Directions::FRONT,
        Direction::Back => Directions::BACK,
        Direction::Up => Directions::UP,
        Direction::Down => Directions::DOWN,
        Direction::Left => Directions::LEFT,
        Direction::Right => Directions::RIGHT,
    };

    let point_uniform = ("point", Box::new(point.as_ref().clone()) as _);

    let mut binder  = Binder::new(
        Vec::new(),
        None,
        program,
        uniforms.chain([point_uniform].into_iter())
    );
    binder.upload();
    binder
}

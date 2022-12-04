use core::borrow::Borrow;
use nalgebra_glm as glm;
use std::prelude::rust_2021::*;
use std::time::Duration;
use crate::{CoordinateSystem, Direction, Directions};


pub struct Camera {
    looking_direction: glm::Vec3,
    position: glm::Vec3,
    aspect_ratio: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32
}

struct RightHandCoordSys {
    front: glm::Vec3,
}

impl RightHandCoordSys {
    const GLOBAL_UP: glm::Vec3 = Directions::UP;

    pub fn new(front: glm::Vec3) -> Self {
        Self { front }
    }

    pub fn direction(&self, direction: &Direction) -> glm::Vec3 {
        let left = self.front.cross(&Self::GLOBAL_UP);
        let up = glm::rotate_vec3(&self.front, 90.0f32.to_radians(), left.borrow());
        match direction {
            Direction::Front => self.front,
            Direction::Back => -self.front,
            Direction::Up => up,
            Direction::Down => -up,
            Direction::Left => -left,
            Direction::Right => left,
        }
    }
}

impl Camera {
    const DEFAULT_FOVY: f32 = 60.0;
    const DEFAULT_Z_NEAR: f32 = 0.1;
    const DEFAULT_Z_FAR: f32 = 100.0;

    const SENSITIVITY: f32 = 0.5;
    const SPEED: f32 = 0.05;

    pub fn angle() -> f32 {
        f32::to_radians(10f32)
    }

    pub fn rotate(&mut self, x_rot: f32, y_rot: f32) {
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, y_rot * Self::SENSITIVITY);
        let coord_sys = RightHandCoordSys::new(self.looking_direction);
        let left = coord_sys.direction(&Direction::Right);
        self.looking_direction = glm::rotate_vec3(&self.looking_direction, x_rot * Self::SENSITIVITY, &left);
    }

    pub fn r#move(&mut self, direction: &Direction) {
        let local = RightHandCoordSys::new(self.looking_direction);
        self.position += local.direction(direction) * Self::SPEED;
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        let looking_point = self.position + self.looking_direction;
        glm::look_at(&self.position, &looking_point, &Directions::UP)
    }

    pub fn perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective(self.aspect_ratio, self.fovy, 0.1, 100.0)
    }

    pub fn new(
        looking_direction: glm::Vec3,
        position: glm::Vec3,
        aspect_ratio: f32,
        fovy: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self { looking_direction, position, aspect_ratio, fovy, z_near, z_far }
    }

    // todo: Make moves dependent of the framerate (time deltas)

}

impl Default for Camera {
    fn default() -> Self {

        let mut viewport = [0; 4];
        unsafe {
            gl::GetIntegerv( gl::VIEWPORT, viewport.as_mut_ptr() );
        }
        let [.., width, height] = viewport;

        Self::new(
            Directions::FRONT,
            glm::vec3(0f32, 0f32, 1f32),
            width as f32 / height as f32,
            Self::DEFAULT_FOVY.to_radians(),
            Self::DEFAULT_Z_NEAR,
            Self::DEFAULT_Z_FAR,
        )
    }
}




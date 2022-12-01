use nalgebra_glm as glm;
use std::prelude::rust_2021::*;
use crate::{CoordinateSystem, Directions};


pub struct Camera {
    looking_direction: glm::Vec3,
    position: glm::Vec3,
    aspect_ratio: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32
}

impl Camera {
    const DEFAULT_FOVY: f32 = 60.0;
    const DEFAULT_Z_NEAR: f32 = 0.1;
    const DEFAULT_Z_FAR: f32 = 100.0;

    pub fn angle() -> f32 {
        f32::to_radians(0.5)
    }

    pub fn rotate(&mut self, x_rot: f32, y_rot: f32) {
        self.looking_direction = glm::rotate_x_vec3(&self.looking_direction, x_rot);
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, y_rot);
    }

    pub fn fixed_rotate_left(&mut self) {
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, Self::angle());
    }

    pub fn fixed_rotate_right(&mut self) {
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, -Self::angle());
    }

    pub fn fixed_rotate_up(&mut self) {
        self.looking_direction = glm::rotate_x_vec3(&self.looking_direction, Self::angle());
    }

    pub fn fixed_rotate_down(&mut self) {
        self.looking_direction = glm::rotate_z_vec3(&self.looking_direction, -Self::angle());
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &self.looking_direction, &Directions::UP)
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

    pub fn move_(&mut self, direction: glm::Vec3) {
        self.position += direction;
    }
}

impl Default for Camera {
    fn default() -> Self {

        let mut viewport = [0; 4];
        unsafe {
            gl::GetIntegerv( gl::VIEWPORT, viewport.as_mut_ptr() );
        }
        let [.., width, height] = viewport;

        Self::new(
            Directions::BACK,
            glm::vec3(4f32, 0f32, 0f32),
            width as f32 / height as f32,
            Self::DEFAULT_FOVY.to_radians(),
            Self::DEFAULT_Z_NEAR,
            Self::DEFAULT_Z_FAR,
        )
    }
}




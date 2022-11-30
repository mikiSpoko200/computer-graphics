use nalgebra_glm as glm;
use crate::Directions;


struct Camera {
    looking_direction: glm::Vec3,
    position: glm::Vec3,
}

impl Camera {
    pub fn angle() -> f32 {
        f32::to_radians(0.5)
    }

    pub fn rotate_left(&mut self) {
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, Self::angle());
    }

    pub fn rotate_right(&mut self) {
        self.looking_direction = glm::rotate_y_vec3(&self.looking_direction, -Self::angle());
    }

    pub fn rotate_up(&mut self) {
        self.looking_direction = glm::rotate_x_vec3(&self.looking_direction, Self::angle());
    }

    pub fn rotate_down(&mut self) {
        self.looking_direction = glm::rotate_z_vec3(&self.looking_direction, -Self::angle());
    }
    //
    // pub fn matrix(&self) -> glm::Mat4 {
    //     // glm::look_at()
    // }
    
    pub fn new(looking_direction: glm::Vec3, position: glm::Vec3) -> Self {
        Self { looking_direction, position }
    }

    pub fn move_(&mut self, direction: glm::Vec3) {
        self.position += direction;
    }
}




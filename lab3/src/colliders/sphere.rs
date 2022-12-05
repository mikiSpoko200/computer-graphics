use nalgebra_glm as glm;

pub struct Sphere {
    center: glm::Vec3,
    r: f32,
}

impl Sphere {
    pub fn center(&self) -> &glm::Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.r
    }
}
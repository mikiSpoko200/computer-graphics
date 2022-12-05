use nalgebra_glm as glm;
use super::sphere::Sphere;

pub trait Collider<C> {
    fn do_collide(&self, other: &C) -> bool;
}

// blanket impl similar to From / Into
// impl<T, C: Collider<T>> Collider<C> for T {
//     fn do_collide(&self, other: &C) -> bool {
//         other.do_collide(self)
//     }
// }

pub struct Segment {
    pub p1: glm::Vec3,
    pub p2: glm::Vec3,
}

impl Segment {
    pub fn spanning_vector(&self) -> glm::Vec3 {
        self.p2 - self.p1
    }
}

// todo: test this
fn distance_squared(s: &Segment, p: &glm::Vec3) -> f32 {
    let spanning_vector = s.spanning_vector();
    let p_p1 = p - s.p1;
    let p_p2 = p - s.p1;

    let spanning_proj = spanning_vector.dot(&p_p1);
    if spanning_proj < 0.0 { return p_p1.norm_squared(); }
    let sv_length_squared = spanning_vector.norm_squared();
    if spanning_proj >= sv_length_squared { return p_p2.norm_squared() }
    p_p1.norm_squared() - spanning_proj * spanning_proj / sv_length_squared
}

pub struct Capsule {
    a: glm::Vec3,
    b: glm::Vec3,
    r: f32,
}

impl Capsule {
    pub fn new(a: glm::Vec3, b: glm::Vec3, r: f32) -> Self {
        Self { a, b, r }
    }

    pub fn spanning_segment(&self) -> Segment {
        Segment { p1: self.a, p2: self.b }
    }
}

// todo: test this
impl Collider<Sphere> for Capsule {
    fn do_collide(&self, other: &Sphere) -> bool {
        let distance_squared = distance_squared(&self.spanning_segment(), other.center());
        let collision_distance = other.radius() + self.r;
        distance_squared <= collision_distance * collision_distance
    }
}
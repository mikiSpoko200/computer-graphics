use core::borrow::Borrow;
use nalgebra_glm as glm;
use std::prelude::rust_2021::*;
use nalgebra_glm::{Mat4, Vec3};
use crate::{Direction, Directions, gl_assert_no_err};


struct RightHandCoordSys {
    front: Vec3,
}

impl RightHandCoordSys {
    const GLOBAL_UP: Vec3 = Directions::UP;

    pub fn new(front: Vec3) -> Self {
        Self { front }
    }

    pub fn direction(&self, direction: &Direction) -> Vec3 {
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

pub trait ViewMatrixProvider {
    fn view_matrix(&self) -> Mat4;
}

pub trait PerspectiveMatrixProvider {
    fn perspective_matrix(&self) -> Mat4;
}

pub trait CameraProvider: PerspectiveMatrixProvider + ViewMatrixProvider { }

impl<C: PerspectiveMatrixProvider + ViewMatrixProvider> CameraProvider for C { }

pub trait KinematicCamera: CameraProvider + Rotatable + FixedMovable { }

impl<K: CameraProvider + Rotatable + FixedMovable> KinematicCamera for K { }

// todo: move to kinematics
pub trait Rotatable {
    fn rotate(&mut self, x_angle: f32, y_angle: f32);
}

pub trait FixedMovable {
    fn is_in_bounds(&self) -> bool { true }

    fn fixed_move(&mut self, direction: &Direction);
}

pub trait Movable {
    fn r#move(&mut self, vector: &Vec3);
}



pub struct FreeRoamingCamera {
    camera: Camera,
}

impl FreeRoamingCamera {
    pub fn get_position(&self) -> Vec3 {
        self.camera.view.position
    }

    pub fn set_position(&mut self, new: Vec3) {
        self.camera.view.position = new;
    }
}

impl From<Camera> for FreeRoamingCamera {
    fn from(camera: Camera) -> Self { Self { camera } }
}

impl FixedMovable for FreeRoamingCamera {
    fn is_in_bounds(&self) -> bool {
        let pos = &self.camera.view.position;
        pos.x > -1.0 && pos.x < 1.0 && pos.y > -1.0 && pos.y < 1.0 && pos.z > -1.0 && pos.z < 1.0
    }

    fn fixed_move(&mut self, direction: &Direction) {
        let position = self.camera.view.position.clone();
        self.camera.r#move(direction);
        if !self.is_in_bounds() {
            self.camera.view.position = position;
        }
    }
}

impl Rotatable for FreeRoamingCamera {
    fn rotate(&mut self, x_angle: f32, y_angle: f32) {
        self.camera.rotate(x_angle, y_angle);
    }
}

impl ViewMatrixProvider for FreeRoamingCamera {
    fn view_matrix(&self) -> Mat4 { self.camera.view_matrix() }
}

impl PerspectiveMatrixProvider for FreeRoamingCamera {
    fn perspective_matrix(&self) -> Mat4 {
        self.camera.perspective_matrix()
    }
}


pub struct HeroShotCamera {
    perspective: CameraPerspectiveState,
    position: Vec3,
    rotation_point: Vec3,
    radius: f32,
}

impl HeroShotCamera {
    const STEP: f32 = 0.99;
    const LOWER_ZOOM_LIMIT: f32 = 0.1;
    const UPPER_ZOOM_LIMIT: f32 = 20.0;

    pub fn new(perspective: CameraPerspectiveState, looking_direction: Vec3, rotation_point: Vec3, radius: f32) -> Self {
        Self {
            perspective,
            position: rotation_point - looking_direction * radius,
            rotation_point,
            radius
        }
    }
}

impl Rotatable for HeroShotCamera {
    fn rotate(&mut self, x_rad: f32, y_rad: f32) {
        let mut transition = self.position - self.rotation_point;
        let coord_sys = RightHandCoordSys::new(-transition);
        let right = coord_sys.direction(&Direction::Right);
        transition = glm::rotate_y_vec3(&transition, y_rad * Camera::SENSITIVITY);
        transition = glm::rotate_vec3(&transition, x_rad * Camera::SENSITIVITY, &right);
        self.position = self.rotation_point + transition;
    }
}

impl FixedMovable for HeroShotCamera {
    fn fixed_move(&mut self, direction: &Direction) {
        let mut transition = self.position - self.rotation_point;
        match direction {
            Direction::Front => transition *= if transition.norm() > Self::LOWER_ZOOM_LIMIT { Self::STEP } else { 1.0 },
            Direction::Back  => transition /= if transition.norm() < Self::UPPER_ZOOM_LIMIT { Self::STEP } else { 1.0 },
            _ => { }
        }
        self.position = self.rotation_point + transition;
    }
}

impl PerspectiveMatrixProvider for HeroShotCamera {
    fn perspective_matrix(&self) -> Mat4 {
        self.perspective.perspective_matrix()
    }
}

impl ViewMatrixProvider for HeroShotCamera {
    fn view_matrix(&self) -> Mat4 {
        glm::look_at(&self.position, &self.rotation_point, &Directions::UP)
    }
}


// general camera
#[derive(Debug, Clone)]
pub struct CameraPerspectiveState {
    aspect_ratio: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32
}

impl Default for CameraPerspectiveState {
    fn default() -> Self {
        let mut viewport = [0; 4];
        gl_assert_no_err!();
        unsafe { gl::GetIntegerv( gl::VIEWPORT, viewport.as_mut_ptr() ); }
        gl_assert_no_err!();
        let [.., width, height] = viewport;

        Self::new(
            width as f32 / height as f32,
            Camera::DEFAULT_FOVY.to_radians(),
            Camera::DEFAULT_Z_NEAR,
            Camera::DEFAULT_Z_FAR,
        )
    }
}

impl CameraPerspectiveState {
    pub fn new(aspect_ratio: f32, fovy: f32, z_near: f32, z_far: f32) -> Self {
        Self { aspect_ratio, fovy, z_near, z_far }
    }
}

impl PerspectiveMatrixProvider for CameraPerspectiveState {
    fn perspective_matrix(&self) -> Mat4 {
        glm::perspective(self.aspect_ratio, self.fovy, self.z_near, self.z_far)
    }
}

#[derive(Debug, Clone)]
pub struct CameraViewState {
    pub looking_direction: Vec3,
    pub position: Vec3,
}

impl Default for CameraViewState {
    fn default() -> Self {
        let looking_direction = Directions::FRONT;
        let position = glm::vec3(0f32, 0f32, 1f32);
        Self { looking_direction, position }
    }
}

impl CameraViewState {
    pub fn new(looking_direction: Vec3, position: Vec3) -> Self {
        Self { looking_direction, position }
    }
}

impl ViewMatrixProvider for CameraViewState {
    fn view_matrix(&self) -> Mat4 {
        let looking_point = self.position + self.looking_direction;
        glm::look_at(&self.position, &looking_point, &Directions::UP)
    }
}

pub struct Camera {
    view: CameraViewState,
    perspective: CameraPerspectiveState,
}

impl Camera {
    const DEFAULT_FOVY: f32 = 60.0;
    const DEFAULT_Z_NEAR: f32 = 0.1;
    const DEFAULT_Z_FAR: f32 = 150.0;

    const SENSITIVITY: f32 = 0.5;
    const SPEED: f32 = 0.05;

    pub fn angle() -> f32 {
        f32::to_radians(10f32)
    }

    pub fn rotate(&mut self, x_rot: f32, y_rot: f32) {
        self.view.looking_direction = glm::rotate_y_vec3(&self.view.looking_direction, y_rot * Self::SENSITIVITY);
        let coord_sys = RightHandCoordSys::new(self.view.looking_direction);
        let right = coord_sys.direction(&Direction::Right);
        self.view.looking_direction = glm::rotate_vec3(&self.view.looking_direction, x_rot * Self::SENSITIVITY, &right);
    }

    pub fn r#move(&mut self, direction: &Direction) {
        let local = RightHandCoordSys::new(self.view.looking_direction);
        self.view.position += local.direction(direction) * Self::SPEED;
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.view.view_matrix()
    }

    pub fn perspective_matrix(&self) -> Mat4 {
        self.perspective.perspective_matrix()
    }

    pub fn new(perspective: CameraPerspectiveState, view: CameraViewState) -> Self {
        Self { perspective, view }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let perspective = CameraPerspectiveState::default();
        let view = CameraViewState::default();
        Self::new(perspective, view)
    }
}




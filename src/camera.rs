use super::math::Vector3f;
use super::display::{Keyboard, Key};

#[derive(Default)]
pub struct Camera {
    pub position: Vector3f,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    const CAMERA_SPEED: f32 = 0.02;

    pub fn move_camera(&mut self, keyboard: &Keyboard) {
        if keyboard.is_pressed(Key::W) {
            self.position.z -= Camera::CAMERA_SPEED;
        } else if keyboard.is_pressed(Key::S) {
            self.position.z += Camera::CAMERA_SPEED;
        } else if keyboard.is_pressed(Key::A) {
            self.position.x -= Camera::CAMERA_SPEED;
        } else if keyboard.is_pressed(Key::D) {
            self.position.x += Camera::CAMERA_SPEED;
        }
    }
}
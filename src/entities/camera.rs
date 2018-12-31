use crate::math::Vector3f;
use crate::display::{Keyboard, Key};
use crate::entities::Player;

pub struct Camera {
    pub position: Vector3f,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    distance_to_player: f32,
    angle_around_player: f32,
}

impl Camera {
    const CAMERA_SPEED: f32 = 0.2;

    pub fn new() -> Camera {
        Camera {
            position: Vector3f::new(0.0, 0.0, 0.0),
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            distance_to_player: 50.0,
            angle_around_player: 0.0,
        }    
    }

    pub fn move_camera(&mut self, keyboard: &Keyboard) {
        // if keyboard.is_pressed(Key::W) {
        //     self.position.z -= Camera::CAMERA_SPEED;
        // } else if keyboard.is_pressed(Key::S) {
        //     self.position.z += Camera::CAMERA_SPEED;
        // } else if keyboard.is_pressed(Key::A) {
        //     self.position.x -= Camera::CAMERA_SPEED;
        // } else if keyboard.is_pressed(Key::D) {
        //     self.position.x += Camera::CAMERA_SPEED;
        // } else if keyboard.is_pressed(Key::Space) {
        //     self.position.y += Camera::CAMERA_SPEED;
        // } else if keyboard.is_pressed(Key::C) {
        //     self.position.y -= Camera::CAMERA_SPEED;
        // }
    }
}
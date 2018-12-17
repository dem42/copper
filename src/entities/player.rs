use std::f32;
use crate::display::{
    Keyboard,
    Display,
    Key,
};
use crate::entities::Entity;

pub struct Player<'a> {
    pub entity: Entity<'a>,
    current_speed: f32,
    current_turn_speed: f32,
    upwards_speed: f32,
    is_in_air: bool,
}

impl<'a> Player<'a> {
    const RUN_SPEED: f32 = 20.0;
    const TURN_SPEED: f32 = 160.0;
    const GRAVITY: f32 = -50.0;
    const JUMP_POWER: f32 = 30.0;

    const TERRAIN_HEIGHT: f32 = 0.0;

    pub fn new(entity: Entity<'a>) -> Player<'a> {
        Player {
            entity,
            current_speed: 0.0,
            current_turn_speed: 0.0,
            upwards_speed: 0.0,
            is_in_air: false,
        }
    }

    pub fn move_player(&mut self, display: &Display) {
        self.check_inputs(display);
        self.entity.increase_rotation(0.0, self.current_turn_speed * display.frame_time_sec, 0.0);
        let distance = self.current_speed * display.frame_time_sec;
        let (y_sin, y_cos) = self.entity.rotation_deg.y.to_radians().sin_cos();
        let dx = distance * y_sin;
        let dz = distance * y_cos;
        self.upwards_speed += Player::GRAVITY * display.frame_time_sec;
        let upwards_dist = self.upwards_speed * display.frame_time_sec;
        self.entity.increase_position(dx, upwards_dist, dz);
        if self.entity.position.y <= Player::TERRAIN_HEIGHT {
            self.entity.position.y = 0.0;
            self.is_in_air = false;
            self.upwards_speed = 0.0;
        }        
    }

    fn check_inputs(&mut self, keyboard: &Keyboard) {
        if keyboard.is_pressed(Key::W) {
            self.current_speed = Player::RUN_SPEED;
        } else if keyboard.is_pressed(Key::S) {
            self.current_speed = -Player::RUN_SPEED;
        } else {
            self.current_speed = 0.0;
        }
        
        if keyboard.is_pressed(Key::A) {
            self.current_turn_speed = -Player::TURN_SPEED;
        } else if keyboard.is_pressed(Key::D) {
            self.current_turn_speed = Player::TURN_SPEED;
        } else {
            self.current_turn_speed = 0.0;
        }

        if keyboard.is_pressed(Key::Space) {
            self.jump();
        }
    }

    fn jump(&mut self) {
        if !self.is_in_air {
            self.upwards_speed = Player::JUMP_POWER;
            self.is_in_air = true;
        }
    }
}
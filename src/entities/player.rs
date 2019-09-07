use std::f32;
use crate::constants::GRAVITY;
use crate::display::{
    Keyboard,
    Display,
    Key,
};
use crate::entities::{
    Entity,
    Ground,
};

pub struct Player {
    pub entity: Entity,
    current_speed: f32,
    current_turn_speed: f32,
    upwards_speed: f32,
    is_in_air: bool,
    pub is_invisible_immovable: bool,
}

impl Player {
    const RUN_SPEED: f32 = 20.0;
    const TURN_SPEED: f32 = 160.0;
    const JUMP_POWER: f32 = 30.0;

    pub fn new(entity: Entity) -> Player {
        Player {
            entity,
            current_speed: 0.0,
            current_turn_speed: 0.0,
            upwards_speed: 0.0,
            is_in_air: false,
            is_invisible_immovable: false,
        }
    }

    pub fn move_player(&mut self, display: &Display, ground: &Ground) {
        if self.is_invisible_immovable {
            return;
        }
        self.check_inputs(display);
        self.entity.increase_rotation(0.0, self.current_turn_speed * display.frame_time_sec, 0.0);
        let distance = self.current_speed * display.frame_time_sec;
        let (y_sin, y_cos) = self.entity.rotation_deg.y.to_radians().sin_cos();
        let dx = distance * y_sin;
        let dz = distance * y_cos;
        self.upwards_speed += GRAVITY * display.frame_time_sec;
        let upwards_dist = self.upwards_speed * display.frame_time_sec;
        self.entity.increase_position(dx, upwards_dist, dz);

        let terrain_height_at_xz = ground.height_at_xz(self.entity.position.x, self.entity.position.z);
        if self.entity.position.y <= terrain_height_at_xz {
            self.entity.position.y = terrain_height_at_xz;
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
            self.current_turn_speed = Player::TURN_SPEED;
        } else if keyboard.is_pressed(Key::D) {
            self.current_turn_speed = -Player::TURN_SPEED;
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
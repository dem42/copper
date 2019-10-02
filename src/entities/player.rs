use crate::constants::GRAVITY;
use crate::display::{
    Keyboard,
    Display,
    Key,
};
use crate::entities::{
    AnimatedEntity,
    Entity,
    Ground,
};
use crate::math::{
    Vector3f,
};
use std::f32;

pub enum PlayerEntityType {
    StaticModelEntity(Entity),
    AnimatedModelEntity(AnimatedEntity),
}

pub struct Player {
    pub entity: PlayerEntityType,
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

    pub fn new_animated(animated_entity: AnimatedEntity) -> Player {
        Player {
            entity: PlayerEntityType::AnimatedModelEntity(animated_entity),
            current_speed: 0.0,
            current_turn_speed: 0.0,
            upwards_speed: 0.0,
            is_in_air: false,
            is_invisible_immovable: false,
        }
    }

    pub fn new(entity: Entity) -> Player {
        Player {
            entity: PlayerEntityType::StaticModelEntity(entity),
            current_speed: 0.0,
            current_turn_speed: 0.0,
            upwards_speed: 0.0,
            is_in_air: false,
            is_invisible_immovable: false,
        }
    }

    pub fn position(&self) -> &Vector3f {
        match &self.entity {
            PlayerEntityType::StaticModelEntity(entity) => &entity.position,
            PlayerEntityType::AnimatedModelEntity(entity) => &entity.position,
        }
    }

    pub fn position_mut(&mut self) -> &mut Vector3f {
        match &mut self.entity {
            PlayerEntityType::StaticModelEntity(entity) => &mut entity.position,
            PlayerEntityType::AnimatedModelEntity(entity) => &mut entity.position,
        }
    }

    pub fn rotation_deg(&self) -> &Vector3f {
        match &self.entity {
            PlayerEntityType::StaticModelEntity(entity) => &entity.rotation_deg,
            PlayerEntityType::AnimatedModelEntity(entity) => &entity.rotation_deg,
        }
    }

    fn increase_position(&mut self, dx: f32, dy: f32, dz: f32) {
        match &mut self.entity {
            PlayerEntityType::StaticModelEntity(entity) => entity.increase_position(dx, dy, dz),
            PlayerEntityType::AnimatedModelEntity(entity) => entity.increase_position(dx, dy, dz),
        }
    }

    fn increase_rotation(&mut self, drx: f32, dry: f32, drz: f32) {
        match &mut self.entity {
            PlayerEntityType::StaticModelEntity(entity) => entity.increase_rotation(drx, dry, drz),
            PlayerEntityType::AnimatedModelEntity(entity) => entity.increase_rotation(drx, dry, drz),
        }
    }

    pub fn move_player(&mut self, display: &Display, ground: &Ground) {
        if self.is_invisible_immovable {
            return;
        }
        self.check_inputs(display);
        self.increase_rotation(0.0, self.current_turn_speed * display.frame_time_sec, 0.0);
        let distance = self.current_speed * display.frame_time_sec;
        let (y_sin, y_cos) = self.rotation_deg().y.to_radians().sin_cos();
        let dx = distance * y_sin;
        let dz = distance * y_cos;
        self.upwards_speed += GRAVITY * display.frame_time_sec;
        let upwards_dist = self.upwards_speed * display.frame_time_sec;
        self.increase_position(dx, upwards_dist, dz);

        let terrain_height_at_xz = ground.height_at_xz(self.position().x, self.position().z);
        if self.position().y <= terrain_height_at_xz {
            self.position_mut().y = terrain_height_at_xz;
            self.is_in_air = false;
            self.upwards_speed = 0.0;
        }
    }

    pub fn is_moving(&self) -> bool {
        self.current_speed > 0.0 || self.is_in_air
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
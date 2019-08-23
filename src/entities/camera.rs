use std::f32;
use crate::math::{
    Quaternion,
    Vector3f,
};
use crate::display::{
    Display,
};
use crate::entities::Player;

pub struct Camera {
    pub position: Vector3f,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub looking_at: Vector3f,
    pub up: Vector3f,
    distance_to_player: f32,
    angle_around_player: f32,
}

impl Camera {

    pub fn new(camera_pitch_dg: f32, distance_to_player: f32) -> Camera {
        Camera {
            position: Vector3f::new(0.0, 0.0, 0.0),
            roll: 0.0,
            pitch: camera_pitch_dg,
            yaw: 0.0,
            distance_to_player,
            angle_around_player: 0.0,
            looking_at: Vector3f::zero(),
            up: Vector3f::new(0.0, 1.0, 0.0),
        }    
    }

    pub fn move_camera(&mut self, display: &Display, player: &Player) {
        self.calc_zoom(display);
        self.calc_pitch(display);
        self.calc_angle_around_player(display);        
        self.update_camera_pos(player);
    }

    pub fn set_to_reflected_ray_camera_origin(&mut self, reflection_plane_y: f32) {        
        self.position.y -= 2.0 * (self.position.y - reflection_plane_y);
        self.pitch = if self.pitch >= 180.0 {
            self.pitch - 360.0
        } else {
            -self.pitch
        }
    }

    fn update_camera_pos(&mut self, player: &Player) {
        let (s, c) = self.pitch.to_radians().sin_cos();
        let (camera_vertical_offset_to_player, camera_horizontal_offset_to_player) = (self.distance_to_player * s, self.distance_to_player * c);        
        
        let (s, c) = (player.entity.rotation_deg.y + self.angle_around_player).to_radians().sin_cos();        
        let (x_offset, z_offset) = (camera_horizontal_offset_to_player * s, camera_horizontal_offset_to_player * c);

        let player_pos = &player.entity.position;
        self.position = Vector3f::new(player_pos.x - x_offset, player_pos.y + camera_vertical_offset_to_player, player_pos.z - z_offset);        
        
        self.looking_at = player.entity.position.clone();
        let q1 = Quaternion::from_angle_axis(-self.pitch, &Vector3f::POS_X_AXIS);
        let q2 = Quaternion::from_angle_axis(player.entity.rotation_deg.y + self.angle_around_player, &Vector3f::POS_Y_AXIS);
        let rot = q2 * q1;
        self.up = Quaternion::rotate_vector(&Vector3f::POS_Y_AXIS, &rot);
                
        self.yaw = player.entity.rotation_deg.y + self.angle_around_player - 180.0; // remove the rotation to get player model to face right way
    }

    fn calc_zoom(&mut self, display: &Display) {
        let zoom_change = display.mouse_pos.d_scroll();
        // negative scroll should move away from player -> therefore in positive z direction in world coord (because positive goes towards you and negative goes into distance)
        self.distance_to_player -= zoom_change as f32;
    }

    fn calc_pitch(&mut self, display: &Display) {
        if display.mouse_pos.is_right_pressed {
            let pitch_change = display.mouse_pos.dy() * 0.1;
            // mouse coords start with (0,0) in bot, left -> positive dy means moving mouse down and we want the inverse mouse control
            // because when we grab the player and pull down on him we want the view to raise above him
            self.pitch += pitch_change as f32; 
        }
    }

    fn calc_angle_around_player(&mut self, display: &Display) {
        if display.mouse_pos.is_left_pressed {
            let angle_around_change = display.mouse_pos.dx() * 0.1;
            // mouse coords start with (0,0) in bot, left -> therefore positive dx means right. we want inverted mouse control like in calc_pitch
            // in this case the angle around player needs to be clockwise
            self.angle_around_player -= angle_around_change as f32; 
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(20.0, 50.0)
    }
}
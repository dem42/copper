use crate::models::{
    SkyboxModel,
    TextureId,
};
use crate::display::{
    Display,
    WallClock,
};

const ROTATE_SPEED: f32 = 1.0;
const DAY_SEGMENTS: f32 = WallClock::DAY_LENGTH / 4.0;

pub struct Skybox {
    pub model: SkyboxModel,
    pub rotation_yaw_deg: f32,
    // this is a hack for scenes that don't want to show a skybox :(
    pub invisible: bool,
}

impl Skybox {
    pub fn new(model: SkyboxModel, rotation_yaw_deg: f32) -> Skybox {
        Skybox {
            model,
            rotation_yaw_deg,
            invisible: false,
        }
    }

    pub fn increase_rotation(&mut self, display: &Display) {
        self.rotation_yaw_deg += ROTATE_SPEED * display.frame_time_sec;
    }

    pub fn get_day_night_textures(&self, wall_clock: &WallClock) -> (TextureId, TextureId, f32) {
        if wall_clock.time_of_day < DAY_SEGMENTS {
            (self.model.night_texture_id, self.model.day_texture_id, wall_clock.time_of_day / DAY_SEGMENTS)
        } else if wall_clock.time_of_day < 2.0 * DAY_SEGMENTS {
            (self.model.day_texture_id, self.model.day_texture_id, (wall_clock.time_of_day - DAY_SEGMENTS) / DAY_SEGMENTS)
        } else if wall_clock.time_of_day < 3.0 * DAY_SEGMENTS {
            (self.model.day_texture_id, self.model.night_texture_id, (wall_clock.time_of_day - 2.0 * DAY_SEGMENTS) / DAY_SEGMENTS)
        } else {
            (self.model.night_texture_id, self.model.night_texture_id, (wall_clock.time_of_day - 3.0 * DAY_SEGMENTS) / DAY_SEGMENTS)
        }
    }
}
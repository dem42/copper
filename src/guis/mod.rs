use crate::math::{
    Vector2f,
};

pub struct Gui {
    pub texture_id: u32,
    pub position: Vector2f,
    // scale relative to screen width/height
    pub scale: Vector2f,
}

impl Gui {
    pub fn new(texture_id: u32, position: Vector2f, scale: Vector2f) -> Gui {
        Gui {
            texture_id,
            position,
            scale,
        }
    }
}
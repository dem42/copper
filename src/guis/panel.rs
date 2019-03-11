use crate::math::{
    Vector2f,
};

pub struct GuiPanel {
    pub texture_id: u32,
    pub position: Vector2f,
    // scale relative to screen width/height
    pub scale: Vector2f,
}

impl GuiPanel {
    pub fn new(texture_id: u32, position: Vector2f, scale: Vector2f) -> GuiPanel {
        GuiPanel {
            texture_id,
            position,
            scale,
        }
    }
}
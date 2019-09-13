use crate::math::{
    Vector2f,
};
use crate::models::TextureId;

pub struct GuiPanel {
    pub texture_id: TextureId,
    pub position: Vector2f,
    // scale relative to screen width/height
    pub scale: Vector2f,
}

impl GuiPanel {
    pub fn new(texture_id: TextureId, position: Vector2f, scale: Vector2f) -> GuiPanel {
        GuiPanel {
            texture_id,
            position,
            scale,
        }
    }
}
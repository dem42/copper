use crate::math::{
    Vector2f,
    Vector3f,
};
use crate::models::TexturedModel;

pub struct Entity<'a> {
    pub model: &'a TexturedModel,
    pub position: Vector3f,
    pub rotation_deg: Vector3f,
    pub scale: f32,
    pub atlas_index: usize,    
}

impl<'a> Entity<'a> {
    pub fn new(model: &'a TexturedModel, position: Vector3f, rotation_deg: Vector3f, scale: f32) -> Entity<'a> {
        Entity {
            model,
            position,
            rotation_deg,
            scale,
            atlas_index: 0,
        }
    }

    pub fn new_with_texture_atlas(model: &'a TexturedModel, position: Vector3f, rotation_deg: Vector3f, scale: f32, atlas_index: usize) -> Entity<'a> {
        Entity {
            model,
            position,
            rotation_deg,
            scale,
            atlas_index,
        }
    }

    pub fn set_position(&mut self, new_pos: &Vector3f) {
        self.position.x = new_pos.x;
        self.position.y = new_pos.y;
        self.position.z = new_pos.z;
    }

    pub fn increase_position(&mut self, dx: f32, dy: f32, dz: f32) {
        self.position.x += dx;
        self.position.y += dy;
        self.position.z += dz;
    }

    pub fn increase_rotation(&mut self, drx: f32, dry: f32, drz: f32) {
        self.rotation_deg.x += drx;
        self.rotation_deg.y += dry;
        self.rotation_deg.z += drz;
    }

    pub fn get_atlas_offset(&self) -> Vector2f {
        let num_rows = self.model.texture.number_of_rows_in_atlas;
        let row = self.atlas_index / num_rows;
        let column = self.atlas_index % num_rows;
        // the texture coordinates are (0,0) to (1,1) -> we want the offsets in this range
        // the offsets say which column/row to shift the original texture coordinates
        let num_rows = num_rows as f32;
        let u_offset = column as f32 / num_rows;
        let v_offset = row as f32 / num_rows;
        Vector2f::new(u_offset, v_offset)
    }
}
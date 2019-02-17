use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::models::{
    WaterModel,
};

pub struct WaterTile {
    pub position: Vector3f,
    pub transform: Matrix4f,
    pub model: WaterModel,
}

impl WaterTile {
    const SIZE: f32 = 200.0;

    pub fn new(position: Vector3f, model: WaterModel) -> Self {
        let transform = Matrix4f::create_transform_matrix(&position, &Vector3f::new(0.0, 0.0, 0.0), WaterTile::SIZE);
        WaterTile {
            position,
            transform,
            model,
        }
    }

    pub fn get_water_height(water_tiles: &Vec<WaterTile>) -> f32 {
        water_tiles[0].position.y
    }
}
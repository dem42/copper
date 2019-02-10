use crate::math::{
    Vector3f,
    Matrix4f,
};
use crate::models::{
    RawModel,
};

pub struct WaterTile<'a> {
    pub position: Vector3f,
    pub transform: Matrix4f,
    pub model: &'a RawModel,
}

impl<'a> WaterTile<'a> {
    const SIZE: f32 = 200.0;

    pub fn new(position: Vector3f, model: &'a RawModel) -> Self {
        let transform = Matrix4f::create_transform_matrix(&position, &Vector3f::new(0.0, 0.0, 0.0), WaterTile::SIZE);
        WaterTile {
            position,
            transform,
            model,
        }
    }
}
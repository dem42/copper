use super::super::math::Vector3f;

pub struct Light {
    pub position: Vector3f,
    pub color: Vector3f,
}

impl Light {
    pub fn new(position: Vector3f, color: Vector3f) -> Light {
        Light {
            position,
            color,
        }
    }
}
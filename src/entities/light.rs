use super::super::math::Vector3f;

pub struct Light {
    pub position: Vector3f,
    pub color: Vector3f,
    pub attenuation: Vector3f,
}

impl Light {
    pub fn new_infinite(position: Vector3f, color: Vector3f) -> Light {
        Light {
            position,
            color,
            attenuation: Vector3f::new(1.0, 0.0, 0.0),       
        }
    }

    pub fn new_point(position: Vector3f, color: Vector3f, attenuation: Vector3f) -> Light {
        Light {
            position,
            color,
            attenuation,       
        }
    }
}
use std::ops::Neg;

#[derive(Debug, Default)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f { x, y, z}
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(mut self) -> Vector3f {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl<'a> Neg for &'a Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Vector3f {
        Vector3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
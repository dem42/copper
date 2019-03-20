use std::ops::{Neg, Index, IndexMut, Add, Sub, AddAssign, Mul, MulAssign};
use std::iter::IntoIterator;
use std::f32;

#[derive(Debug, Default, Clone)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4f {
    pub const ZERO: Vector4f = Vector4f {x: 0.0, y: 0.0, z: 0.0, w: 0.0};

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4f {
        Vector4f { x, y, z, w}
    }
}

impl IntoIterator for Vector4f {
    type Item = f32;
    type IntoIter = ::std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z, self.w].into_iter()
    }
}

impl Index<usize> for Vector4f {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Cannot index 4 vec with {}", index)
        }
    }
}

impl IndexMut<usize> for Vector4f {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Cannot index 4 vec with {}", index)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub const ZERO: Vector3f = Vector3f {x: 0.0, y: 0.0, z: 0.0};
    pub const POS_X_AXIS: Vector3f = Vector3f {x: 1.0, y: 0.0, z: 0.0};
    
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f { x, y, z}
    }

    pub fn zero() -> Vector3f {
        Vector3f::ZERO.clone()
    }

    pub fn onto_project(&self, other: &Vector3f) -> Vector3f {
        let dot = self.dot_product(other);
        let len = self.dot_product(&self);
        let factor = dot / len;
        Vector3f::new(factor * self.x, factor * self.y, factor * self.z)
    }

    pub fn length(&self) -> f32 {
        let sq_sum = self.x * self.x + self.y * self.y + self.z * self.z;
        sq_sum.sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn dot_product(&self, other: &Vector3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_prod(&self, o: &Vector3f) -> Vector3f {
        let i_axis_coef = self.y * o.z - o.y * self.z;
        let j_axis_coef = self.x * o.z - o.x * self.z;
        let k_axis_coef = self.x * o.x - o.x * self.y;
        Vector3f::new(i_axis_coef, -j_axis_coef, k_axis_coef)
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

impl Add for Vector3f {
    type Output = Vector3f;

    fn add(mut self, other: Vector3f) -> Vector3f {
        self.x += other.x;
        self.y += other.y; 
        self.z += other.z;
        self
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(mut self, other: Vector3f) -> Vector3f {
        self.x -= other.x;
        self.y -= other.y; 
        self.z -= other.z;
        self
    }
}

impl AddAssign<&Vector3f> for Vector3f {
    fn add_assign(&mut self, other: &Vector3f) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(mut self, other: f32) -> Vector3f {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }
}

impl MulAssign<f32> for Vector3f {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl IntoIterator for Vector3f {
    type Item = f32;
    type IntoIter = ::std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f { x, y, }
    }
    pub fn length(&self) -> f32 {
        let sq_sum = self.x * self.x + self.y * self.y;
        sq_sum.sqrt()
    }
}

impl IntoIterator for Vector2f {
    type Item = f32;
    type IntoIter = ::std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y].into_iter()
    }
}
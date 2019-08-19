use crate::math::{
    Matrix4f,
    Vector3f,
};
use std::ops::{
    Index,
    Mul,
};

/**
 * Representation of quaternions. 
 * Unit quaternions are useful for representing a rotation around an arbitrary axis because they are easily composable (stackable)
 * using the multiplication operation. The final result can then be turned into a single rotation matrix that represents a rotation
 * that would be the result of all the other rotations. (also no gimbal lock)
 */
#[derive(Debug, Clone)]
pub struct Quaternion {
    a: f32,
    v: Vector3f,
}

impl Quaternion {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Quaternion {
            a,
            v: Vector3f::new(b, c, d),
        }
    }

    pub fn from_angle_axis(alpha_deg: f32, axis: &Vector3f) -> Self {
        let (sin, cos) = (alpha_deg / 2.0).to_radians().sin_cos();
        let v = sin * axis;
        Quaternion {
            a: cos,
            v,
        }
    }

    pub fn as_rot_mat(&self) -> Matrix4f {
        let mut res = Matrix4f::zeros();
        let s = self.length();
        let (qr, qi, qj, qk) = (self.a, self.v.x, self.v.y, self.v.z);        
        res[0][0] = 1.0 - 2.0*s*(qj*qj + qk*qk);
        res[0][1] = 2.0*s*(qi*qj - qk*qr);
        res[0][2] = 2.0*s*(qi*qk + qj*qr);  
        res[1][0] = 2.0*s*(qi*qj + qk*qr);
        res[1][1] = 1.0 - 2.0*s*(qi*qi + qk*qk);
        res[1][2] = 2.0*s*(qj*qk - qi*qr);
        res[2][0] = 2.0*s*(qi*qk - qj*qr);
        res[2][1] = 2.0*s*(qj*qk + qi*qr);
        res[2][2] = 1.0 - 2.0*s*(qi*qi + qj*qj);
        res[3][3] = 1.0;
        res
    }

    pub fn normalize(&mut self) {
        let l = 1.0 / self.length();
        self.a *= l;
        for i in 0..3 {
            self.v[i] *= l;
        }
    }

    pub fn length(&self) -> f32 {
        (self.a * self.a + self.v.dot_product(&self.v)).sqrt()
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            a: self.a,
            v: -1.0 * &self.v,
        }
    }

    pub fn reciprocal(&self) -> Quaternion {
        let s = self.length();
        let s = 1.0 / (s * s);
        let mut conj = self.conjugate();
        conj.a *= s;
        conj.v = s * conj.v;
        conj
    }

    pub fn rotate_vector(vector: &Vector3f, quaternion: &Quaternion) -> Vector3f {
        let v_as_q = Quaternion { a: 0.0, v: vector.clone() };
        let recip = quaternion.reciprocal();
        let res = (quaternion * v_as_q) * recip;        
        res.v
    }
}

fn mult(a0: f32, v0: &Vector3f, a1: f32, v1: &Vector3f) -> Quaternion {
    let a = a0 * a1 - v0.dot_product(v1);    
    let v = (a0 * v1) + (a1 * v0) + v0.cross_prod(v1);
    Quaternion {
        a,
        v, 
    }
}

impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, o: &Quaternion) -> Quaternion {
        mult(self.a, &self.v, o.a, &o.v)
    }
}

impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, o: Quaternion) -> Quaternion {
        mult(self.a, &self.v, o.a, &o.v)
    }
}

impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, o: &Quaternion) -> Quaternion {
        mult(self.a, &self.v, o.a, &o.v)
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, o: Quaternion) -> Quaternion {
        mult(self.a, &self.v, o.a, &o.v)
    }
}

impl Index<usize> for Quaternion {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.a,
            1 => &self.v.x,
            2 => &self.v.y,
            3 => &self.v.z,
            _ => panic!("Cannot index 4 quaternion with {}", index)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use crate::math::Vector4f;

    #[test]
    fn test_hamilton_product() {
        let q1 = Quaternion::new(2.3, 1.0, 4.0, -1.2);
        let q2 = Quaternion::new(-4.2, 3.2, -10.1, 12.0);

        let res = q1 * q2;
        let expected = [41.94,39.04,-55.87,9.74];

        for i in 0..4 {
            assert_f32_eq!(expected[i], res[i], test_constants::EPS_MEDIUM, &format!("Mismatch on pos: {}.", i));
        }        
    }

    #[test]
    fn test_to_rot_mat() {
        let mut q = Quaternion::new(4.0, 1.0, 2.0, -3.0);
        q.normalize();
        let expected = [0.7302967433402214, 0.18257418583505536, 0.3651483716701107, -0.5477225575051661];
        for i in 0..4 {
            assert_f32_eq!(expected[i], q[i], test_constants::EPS_MEDIUM, &format!("Mismatch on pos: {}.", i));
        }
        let mat = q.as_rot_mat();
        let expected = Matrix4f::new([[0.13333333333333353, 0.9333333333333332, 0.33333333333333326, 0.0],
            [-0.6666666666666666, 0.3333333333333335, -0.6666666666666665, 0.0],
            [-0.7333333333333332, -0.13333333333333336, 0.6666666666666667, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
        );
        for i in 0..4 {
            for j in 0..4 {
                assert_f32_eq!(expected[i][j], mat[i][j], test_constants::EPS_MEDIUM, &format!("Mismatch on: ({},{}).", i, j));
            }
        }
    }

    #[test]
    fn test_vec_rotation() {
        let mut q = Quaternion::new(4.0, 1.0, 2.0, -3.0);
        let tv = Vector3f::new(2.3, 1.2, -0.8);
        q.normalize();        
        let mat = q.as_rot_mat();

        let mut expected = mat.transform(&Vector4f::from_point(&tv)).xyz();
        expected.normalize();
        let mut result = Quaternion::rotate_vector(&tv, &q);
        result.normalize();

        for i in 0..3 {
            assert_f32_eq!(expected[i], result[i], test_constants::EPS_MEDIUM, &format!("Mismatch on: {}.", i));            
        }
    }
}
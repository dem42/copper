use crate::math::{
    Matrix4f,
    Vector3f,
};
use std::ops::{
    Add,
    Sub,
    Index,
    Mul,
    Neg,    
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

    pub fn identity() -> Self {
        Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Quaternion {
            a,
            v: Vector3f::new(b, c, d),
        }
    }

    pub fn from_angle_axis(alpha_deg: f32, axis: &Vector3f) -> Self {
        let (sin, cos) = (alpha_deg / 2.0).to_radians().sin_cos();
        let v = sin * axis;
        let mut q = Quaternion {
            a: cos,
            v,
        };
        q.normalize();
        q
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

    pub fn from_rot_mat(orthogonal_mat: &Matrix4f) -> Quaternion {
        // we use the face that the trace sums to 3 - 4*(x^2 + y^2 + z^2) which since we want a unit quaternion
        // is |q| = w^2 + x^2 + y^2 + z^2 = 1 => 4*w^2 = 4 - 4*(x^2 + y^2 + z^2)
        // so we know the w (first component) is
        let r = orthogonal_mat.trace(); // since our orthogonal mat has 1 in [3][3] we don't need to add 1 to get 4 - 4(xx+yy+zz)
        let w = r.sqrt() / 2.0;
        // if we change the signs on the diagonal elements then we can build terms that have the following form
        // Qxx - Qyy - Qzz = -1 + 4xx
        // or similar for other sign combinations -> from this we can compute the x
        // but since sqrt has two possible solutions (pos and neg) we need to find the sign from the a combinations of entries that
        // only depend on x and w (since we took positive w)
        let x = Self::copysign((1.0 + orthogonal_mat[0][0] - orthogonal_mat[1][1] - orthogonal_mat[2][2]).sqrt() / 2.0, orthogonal_mat[2][1] - orthogonal_mat[1][2]);
        let y = Self::copysign((1.0 - orthogonal_mat[0][0] + orthogonal_mat[1][1] - orthogonal_mat[2][2]).sqrt() / 2.0, orthogonal_mat[0][2] - orthogonal_mat[2][0]);
        let z = Self::copysign((1.0 - orthogonal_mat[0][0] - orthogonal_mat[1][1] + orthogonal_mat[2][2]).sqrt() / 2.0, orthogonal_mat[1][0] - orthogonal_mat[0][1]);

        Quaternion::new(w, x, y, z)
    }

    fn copysign(x: f32, y: f32) -> f32 {
        y.signum() * x.abs()
    }

    pub fn normalize(&mut self) {
        let l = 1.0 / self.length();
        self.a *= l;
        for i in 0..3 {
            self.v[i] *= l;
        }
    }

    pub fn normalized(&self) -> Quaternion {
        let mut normed = self.clone();
        let l = 1.0 / self.length();
        normed.a *= l;
        for i in 0..3 {
            normed.v[i] *= l;
        }
        normed
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
        let quaterion_conj = quaternion.reciprocal();
        let res = quaternion * v_as_q * quaterion_conj;
        res.v
    }

    // reflection through origin
    pub fn reflect_point(point: &Vector3f, quaternion: &Quaternion) -> Vector3f {
        let v_as_q = Quaternion { a: 0.0, v: point.clone() };        
        let res = quaternion * v_as_q * quaternion;
        res.v
    }

    pub fn slerp(start: &Quaternion, end: &Quaternion, alpha: f32) -> Quaternion {        
        // i.e kind of a half-intuitive explanation
        // the spherical interpolation is a linear interpolation with weights based on the sine of the angle
        // with unit quaternions whose powers we can write as q^t = cos t*theta + v sin t*theta
        // we see that we can get the sin theta factor easily out of the quaternion and so we can use them to do the slerp efficiently
        let q1 = start.normalized();
        let mut q2 = end.normalized();
        
        let mut real_part = q1.a * q2.a + q1.v.dot_product(&q2.v);

        if real_part < 0.0 {
            real_part = -real_part;
            q2 = -q2;
        }
        const DOT_THRESHOLD: f32 = 0.9995;
        if real_part > DOT_THRESHOLD {
            let step = alpha*(q2 - &q1);
            let result = q1 + step;
            return result.normalized();
        }

        let theta_0 = real_part.acos();
        let theta = theta_0 * alpha;
        let (sin_theta, cos_theta) = theta.sin_cos();
        let sin_theta_0 = theta_0.sin();

        let s0 = cos_theta - real_part * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0; 
                
        (s0 * q1) + (s1 * q2)
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(mut self, o: Quaternion) -> Quaternion {
        self.a += o.a;
        self.v = self.v + o.v;
        self
    }
}

impl Sub<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(mut self, o: &Quaternion) -> Quaternion {
        self.a -= o.a;
        self.v = self.v - &o.v;
        self
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

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, mut o: Quaternion) -> Quaternion {
        o.a *= self;
        o.v = self * o.v;
        o
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(mut self) -> Quaternion {        
        self.a = -self.a;
        self.v.x = -self.v.x;
        self.v.y = -self.v.y;
        self.v.z = -self.v.z;
        self
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
    fn test_hamilton_product_with_0() {
        let q1 = Quaternion::new(0.1, -0.1, 0.0, 0.0);
        let q2 = Quaternion::new(0.0, 0.0, 1.0, 0.0);        

        let res = q1 * q2;
        let expected = [0.0,0.0,0.1,-0.1];

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
    fn test_from_rot_mat() {
        let q = Quaternion::from_angle_axis(90.0, &Vector3f::new(0.3, -3.1, -1.0));
        let r = q.as_rot_mat();
        let q_n = Quaternion::from_rot_mat(&r);        
        
        assert_f32_eq!(q.a, q_n.a, test_constants::EPS_MEDIUM, "Mismatch on: a");
        assert_f32_eq!(q.v[0], q_n.v[0], test_constants::EPS_MEDIUM, "Mismatch on: x");
        assert_f32_eq!(q.v[1], q_n.v[1], test_constants::EPS_MEDIUM, "Mismatch on: y");
        assert_f32_eq!(q.v[2], q_n.v[2], test_constants::EPS_MEDIUM, "Mismatch on: z");
    }

    #[test]
    fn test_vec_rotation() {
        let mut q = Quaternion::new(4.0, 1.0, 2.0, -3.0);
        let tv = Vector3f::new(2.3, 1.2, -0.8);
        q.normalize();        
        let mat = q.as_rot_mat();

        let mut expected = mat.transform(&Vector4f::point(&tv)).xyz();
        expected.normalize();
        let mut result = Quaternion::rotate_vector(&tv, &q);
        result.normalize();

        for i in 0..3 {
            assert_f32_eq!(expected[i], result[i], test_constants::EPS_MEDIUM, &format!("Mismatch on: {}.", i));            
        }
    }

    #[test]
    fn test_quaternion_slerp() {        
        let q1 = Quaternion::from_angle_axis(30.0, &Vector3f::new(1.2, 0.3, -3.0));
        let q2 = Quaternion::from_angle_axis(50.0, &Vector3f::new(10.2, 4.5, -1.0));

        let res = Quaternion::slerp(&q1, &q2, 0.25);
        // expected result obtained from Martin Bakers article where there is a slerp calculator
        // http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/slerp/index.htm
        assert_f32_eq!(res.a, 0.6819695635664609, test_constants::EPS_PRECISE);
        assert_f32_eq!(res.v.x, 0.4754162942171991, test_constants::EPS_PRECISE);
        assert_f32_eq!(res.v.y, 0.1713716492136952, test_constants::EPS_PRECISE);
        assert_f32_eq!(res.v.z, -0.5287046288235048, test_constants::EPS_PRECISE);
    }
}
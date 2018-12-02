use std::ops::{Index, IndexMut};
use super::Vector3f;
use std::f32;

#[derive(Debug)]
pub struct Matrix4f {
    data: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn create_transform_matrix(translation: &Vector3f, rot_xyz_degrees: &Vector3f, scale: f32) -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            data[i][i] = 1.0;
        }
        let mut transform_mat = Matrix4f { data };
        transform_mat.translate(translation);
        transform_mat.rotate_tait_bryan_zyx(rot_xyz_degrees);
        transform_mat.scale(&Vector3f::new(scale, scale, scale));
        transform_mat
    }

    pub fn translate(&mut self, translation: &Vector3f) {
        for i in 0..4 {            
            self.data[i][3] += translation.x * self.data[i][0] + translation.y * self.data[i][1] + translation.z * self.data[i][2]; 
        }
    }

    pub fn scale(&mut self, scale: &Vector3f) {
        for i in 0..4 {            
            self.data[i][0] *= scale.x;
            self.data[i][1] *= scale.y;
            self.data[i][2] *= scale.z;
        }
    }

    // multiply 3 rotation matrices with each other in zyx order
    pub fn rotate_tait_bryan_zyx(&mut self, rot_xyz_degrees: &Vector3f) {
        let mut rot_mat = [[0.0f32; 4]; 4];
        rot_mat[3][3] = 1.0;
        let (sc, cc) = rot_xyz_degrees.x.to_radians().sin_cos();
        let (sb, cb) = rot_xyz_degrees.y.to_radians().sin_cos();
        let (sa, ca) = rot_xyz_degrees.z.to_radians().sin_cos();
        
        rot_mat[0][0] = cb*cc;
        rot_mat[0][1] = cc*sa*sb - ca*sc;
        rot_mat[0][2] = ca*cc*sb + sa*sc;

        rot_mat[1][0] = cb*sc;
        rot_mat[1][1] = sa*sb*sc + ca*cc;
        rot_mat[1][2] = ca*sb*sc - cc*sa;

        rot_mat[2][0] = -sb;
        rot_mat[2][1] = cb*sa;
        rot_mat[2][2] = ca*cb;
        let rot = Matrix4f { data: rot_mat };
        self.multiply_in_place(&rot);
    }

    pub fn multiply_in_place(&mut self, other: &Matrix4f) {
        let mut res_mat = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res_mat[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        self.data = res_mat;
    }

    pub fn data(&self) -> &[[f32; 4]; 4] {
        &self.data
    }
}

impl Index<usize> for Matrix4f {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &[f32; 4] {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4f {

    fn index_mut(&mut self, index: usize) -> &mut [f32; 4] {
        &mut self.data[index]
    }
}

use std::ops::{Index, IndexMut};
use super::{
    Vector2f,
    Vector3f,
    Vector4f,
};
use std::f32;
use super::super::entities::Camera;

#[derive(Debug)]
pub struct Matrix4f {
    data: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn identity() -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            data[i][i] = 1.0;
        }
        Matrix4f {
            data,
        }
    }

    pub fn create_gui_transform_matrix(translation: &Vector2f, scale: &Vector2f) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.translate(&Vector3f::new(translation.x, translation.y, 0.0));        
        transform_mat.scale(&Vector3f::new(scale.x, scale.y, 1.0));
        transform_mat
    }

    pub fn create_transform_matrix(translation: &Vector3f, rot_xyz_degrees: &Vector3f, scale: f32) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.translate(translation);
        transform_mat.rotate_tait_bryan_zyx(rot_xyz_degrees);
        transform_mat.scale(&Vector3f::new(scale, scale, scale));
        transform_mat
    }

    pub fn create_projection_matrix(near_plane: f32, far_plane: f32, fov_horizontal_degs: f32, aspect_ratio: f32) -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        let tan_fov_half = (fov_horizontal_degs.to_radians() / 2.0).tan();
        data[0][0] = 1.0 / tan_fov_half;
        data[1][1] = aspect_ratio / tan_fov_half;
        data[2][2] = -(far_plane + near_plane) / (far_plane - near_plane);
        data[2][3] = (2.0 * far_plane * near_plane) / (far_plane - near_plane);
        data[3][2] = -1.0;

        Matrix4f {
            data,
        }
    }

    // view matrix makes objects move closer to the camera as we move towards them since it includes the negative of the camera translation
    // we dont want the skybox to move as we move around (but we do want it to rotate) so we zero out the translation
    pub fn create_skybox_view_matrix(camera: &Camera, skybox_rotation_deg: f32) -> Matrix4f {        
        // since our up direction is y not z we need to swap yaw and pitch
        let rotation_xyz_degrees = Vector3f::new(camera.roll, camera.yaw + skybox_rotation_deg, camera.pitch);
        let mut view_mat = Matrix4f::identity(); 
        // inverse to transform matrix -> first rotate around xyz (not zyx like normal), then translate by negative translation       
        view_mat.rotate_tait_bryan_xyz(&rotation_xyz_degrees);
        view_mat
    }

    pub fn create_view_matrix(camera: &Camera) -> Matrix4f {
        let translation = &camera.position;
        // since our up direction is y not z we need to swap yaw and pitch
        let rotation_xyz_degrees = Vector3f::new(camera.roll, camera.yaw, camera.pitch);
        let mut view_mat = Matrix4f::identity(); 
        // inverse to transform matrix -> first rotate around xyz (not zyx like normal), then translate by negative translation       
        view_mat.rotate_tait_bryan_xyz(&rotation_xyz_degrees);
        view_mat.translate(&(-translation));
        view_mat
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

    // multiply 3 rotation matrices with each other in xyz order
    pub fn rotate_tait_bryan_xyz(&mut self, rot_xyz_degrees: &Vector3f) {
        let mut rot_mat = [[0.0f32; 4]; 4];
        rot_mat[3][3] = 1.0;
        let (sc, cc) = rot_xyz_degrees.x.to_radians().sin_cos();
        let (sb, cb) = rot_xyz_degrees.y.to_radians().sin_cos();
        let (sa, ca) = rot_xyz_degrees.z.to_radians().sin_cos();
        
        rot_mat[0][0] = cb*cc;
        rot_mat[0][1] = -cb*sc;
        rot_mat[0][2] = sb;

        rot_mat[1][0] = cc*sa*sb + ca*sc;
        rot_mat[1][1] = -sa*sb*sc + ca*cc;
        rot_mat[1][2] = -cb*sa; 

        rot_mat[2][0] = -ca*cc*sb + sa*sc;
        rot_mat[2][1] = ca*sb*sc + cc*sa;
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
    
    fn cofactor_mat(&self) -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mat3 = Matrix3f::ij_minor(i, j, self);
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                data[i][j] = sign * self.data[i][j] * mat3.determinant();
            }
        }
        Matrix4f {
            data,
        }
    }

    fn abjugate_mat(&self) -> Matrix4f {
        let mut cofactor = self.cofactor_mat();
        cofactor.transpose();
        cofactor
    }

    pub fn transform(&self, vec: &Vector4f) -> Vector4f {
        let mut res = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        for i in 0..4 {
            for j in 0..4 {
                res[i] += self.data[i][j] * vec[j];
            }
        }
        res
    }

    pub fn transpose(&mut self) {
        for i in 0..3 {
            for j in (i+1)..4 {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[j][i];
                self.data[j][i] = temp;
            }
        }        
    }

    pub fn inverse(&self) -> Matrix4f {
        let mut abjugate = self.abjugate_mat();
        let determinant = self.determinant();
        let fac = 1.0 / determinant;
        for i in 0..4 {
            for j in 0..4 {
                abjugate.data[i][j] *= fac;
            }
        }
        abjugate
    }

    pub fn determinant(&self) -> f32 {
        let mut res = 0.0;
        let mut sign = 1.0;
        for i in 0..4 {
            let mat3 = Matrix3f::minor(i, self);
            let det = mat3.determinant();
            res += sign * self.data[0][i] * det; 
            sign = -sign;
        }
        res  
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


#[derive(Debug)]
pub struct Matrix3f {
    data: [[f32; 3]; 3],
}

impl Matrix3f {
    pub fn minor(col: usize, mat4: &Matrix4f) -> Matrix3f {
        Matrix3f::ij_minor(0, col, mat4)
    }

    pub fn ij_minor(row: usize, col: usize, mat4: &Matrix4f) -> Matrix3f {
        let mut data = [[0.0f32; 3]; 3];
        let mut nc = 0;        
        for j in 0..4 {
            if j == col {
                continue
            }
            let mut nr = 0;
            for i in 0..4 {
                if i == row {
                    continue
                }
                data[nr][nc] = mat4[i][j];
                nr += 1;
            }                        
            nc += 1;
        }
        Matrix3f {
            data,
        }
    }

    pub fn determinant(&self) -> f32 {        
        let minor_a = self.data[0][0] * (self.data[1][1]*self.data[2][2] - self.data[1][2]*self.data[2][1]);
        let minor_b = self.data[0][1] * (self.data[1][0]*self.data[2][2] - self.data[1][2]*self.data[2][0]);
        let minor_c = self.data[0][2] * (self.data[1][0]*self.data[2][1] - self.data[1][1]*self.data[2][0]);
        minor_a - minor_b + minor_c
    }
}

#[derive(Debug)]
pub struct Matrix2f {
    data: [[f32; 2]; 2],
}

impl Matrix2f {
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.data[0][0]*self.data[1][1] - self.data[0][1]*self.data[1][0]        
    }
}
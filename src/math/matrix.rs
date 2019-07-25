use std::ops::{Index, IndexMut, Mul};
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

    pub fn create_particle_transform_matrix(translation: &Vector3f, rotation_z_deg: f32, scale: f32, view_matrix: &Matrix4f) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.translate(translation);
        // the 3x3 top left corner of view matrix is the camera rotation (no scale .. camera doesnt scale) -> therefore it is an orthonormal transform matrix
        // since it's orthonormal we can cancel it (make it Identity) by multiplying with it's transpose -> so before rotate scale set original rotation to be inverse of view rotation
        for i in 0..3 {
            for j in 0..3 {
                transform_mat.data[i][j] = view_matrix.data[j][i];
            }
        }
        transform_mat.rotate_tait_bryan_zyx(&Vector3f::new(0.0, 0.0, rotation_z_deg));
        transform_mat.scale(&Vector3f::new(scale, scale, scale));
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

    pub fn create_ortho_projection_matrix(frustum_width: f32, frustum_heigh: f32, frustum_length: f32) -> Matrix4f {         
        let mut data = [[0.0f32; 4]; 4];        
        data[0][0] = 2.0 / frustum_width;
        data[1][1] = 2.0 / frustum_heigh;
        data[2][2] = -2.0 / frustum_length;
        data[3][3] = 1.0;
        Matrix4f {
            data,
        }
    }

    // view matrix makes objects move closer to the camera as we move towards them since it includes the negative of the camera translation
    // we dont want the skybox to move as we move around (but we do want it to rotate) so we zero out the translation
    pub fn create_skybox_view_matrix(camera: &Camera, skybox_rotation_deg: f32) -> Matrix4f {        
        // pitch is the rotation against transverse axis (pointing to right) -> x axis is right
        // yaw is the rotation against vertical axis (pointing up) -> y axis is up
        // roll is the rotation against longitudal axis (pointing forward) -> z axis is forward
        let rotation_xyz_degrees = Vector3f::new(camera.pitch, camera.yaw + skybox_rotation_deg, camera.roll);
        let mut view_mat = Matrix4f::identity(); 
        // inverse to transform matrix -> first rotate around xyz (not zyx like normal), then translate by negative translation       
        view_mat.rotate_tait_bryan_xyz(&rotation_xyz_degrees);
        view_mat
    }

    pub fn create_view_matrix(camera: &Camera) -> Matrix4f {
        let translation = &camera.position;
        // pitch is the rotation against transverse axis (pointing to right) -> x axis is right
        // yaw is the rotation against vertical axis (pointing up) -> y axis is up
        // roll is the rotation against longitudal axis (pointing forward) -> z axis is forward
        let rotation_xyz_degrees = Vector3f::new(camera.pitch, camera.yaw, camera.roll);
        let mut view_mat = Matrix4f::identity(); 
        // inverse to transform matrix -> first rotate around xyz (not zyx like normal), then translate by negative translation       
        view_mat.rotate_tait_bryan_xyz(&rotation_xyz_degrees);
        view_mat.translate(&(-translation));
        view_mat
    }

    pub fn calculate_rotation_from_rpy(roll: f32, pitch: f32, yaw: f32) -> Matrix4f {
        let mut result = Matrix4f::identity();
        // rotate in the opposite order to get inverse rotation (camera rotates inversely to model transform)
        let rotation_xyz_degrees = Vector3f::new(pitch, yaw, roll);
        result.rotate_tait_bryan_xyz(&rotation_xyz_degrees);
        result
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
        let (sa, ca) = rot_xyz_degrees.x.to_radians().sin_cos();
        let (sb, cb) = rot_xyz_degrees.y.to_radians().sin_cos();
        let (sc, cc) = rot_xyz_degrees.z.to_radians().sin_cos();
        
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
        let (sa, ca) = rot_xyz_degrees.x.to_radians().sin_cos();
        let (sb, cb) = rot_xyz_degrees.y.to_radians().sin_cos();
        let (sc, cc) = rot_xyz_degrees.z.to_radians().sin_cos();
        
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
                data[i][j] = sign * mat3.determinant();
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
    pub fn new() -> Matrix2f {
        let data = [[0f32; 2]; 2];
        Matrix2f { data }
    }

    pub fn inverse(&self) -> Matrix2f {
        let mut data = [[0f32; 2]; 2];
        let fac = 1.0 / self.determinant();
        data[0][0] = self.data[1][1] * fac;
        data[0][1] = -self.data[0][1] * fac;
        data[1][0] = -self.data[1][0] * fac;
        data[1][1] = self.data[0][0] * fac;
        Matrix2f { data }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.data[0][0]*self.data[1][1] - self.data[0][1]*self.data[1][0]        
    }
}

impl Index<usize> for Matrix2f {
    type Output = [f32; 2];

    fn index(&self, index: usize) -> &[f32; 2] {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix2f {

    fn index_mut(&mut self, index: usize) -> &mut [f32; 2] {
        &mut self.data[index]
    }
}

impl Mul<Matrix4f> for &Matrix4f {
    type Output = Matrix4f;

    fn mul(self, mut other: Matrix4f) -> Matrix4f {
        let mut res = [[0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res[i][j] += self.data[i][k] * other.data[k][j];
                }
            }    
        }
        other.data = res;
        other     
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EPS_PRECISE: f32 = 1e-6;
    const EPS_MEDIUM: f32 = 1e-5;
    const EPS_BAD: f32 = 1e-2;

    macro_rules! assert_f32_eq {
        ($left:expr, $right:expr, $eps:expr) => (assert!(($left - $right).abs() < $eps, format!("Left: {}, Right: {}.", $left, $right)););
        ($left:expr, $right:expr, $eps:expr, $msg:expr) => (assert!(($left - $right).abs() < $eps, format!("{}. Left: {}, Right: {}.", $msg, $left, $right));)
    }

    #[test]
    fn matrix_mul_4x4() {
        let m1 = Matrix4f {data: [[-1.3, 10.0, 2.8, 2.0], [-2.0, 3.0, -13.0, 9.0], [-1.4, 4.5, 0.0, 3.2], [-1.0, -2.0, 3.0, 5.0]] };
        let m2 = Matrix4f {data: [[-0.3, -7.0, 12.8, 2.0], [3.0, 3.0, 3.0, 3.0], [-4.0, -4.5, 4.0, 4.5], [-11.0, -22.0, 33.0, 55.0]] };        
        let result = &m1 * m2;
        
        let expected = Matrix4f {data: [[-2.81, -17.5, 90.560, 150.0], [-37.4, -116.5, 228.4, 441.5], [-21.28, -47.10, 101.18, 186.7], [-72.7, -122.50, 158.20, 280.5]] };  
        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(result[r][c], expected[r][c], tests::EPS_MEDIUM, format!("(r,c)=({},{}) mismatch", r, c));
            }
        }
    }

    #[test]
    fn test_determinant_3x3_a() {
        let data = [[-233.1, 10.0, 2.8], [-12.0, 12.0, -13.0], [-1.4, 4.5, 0.0]];
        let m1 = Matrix3f { data, };
        assert_f32_eq!(-13558.51, m1.determinant(), tests::EPS_BAD);
    }

    #[test]    
    fn test_determinant_4x4_a() {
        let data = [[1.0, 2.3, 12.2, 7.0], [-4.3, -233.1, 10.0, 2.8], [12.0, -12.0, 12.0, -13.0], [1.2, -1.4, 4.5, 0.0]];
        let m1 = Matrix4f { data, };
        assert_f32_eq!(-33206.8290000003, m1.determinant(), tests::EPS_BAD);              
    }

    #[test]
    fn test_transpose_4x4_a() {
        let data = [[1.0, 2.3, 12.2, 7.0], [-4.3, -233.1, 10.0, 2.8], [12.0, -12.0, 12.0, -13.0], [1.2, -1.4, 4.5, 0.0]];
        let mut m1 = Matrix4f { data, };
        m1.transpose();
        let expected = [[1.0, -4.3, 12.0, 1.2], [2.3, -233.1, -12.0, -1.4], [12.2, 10.0, 12.0, 4.5], [7.0, 2.8, -13.0, 0.0]];

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(m1[r][c], expected[r][c], tests::EPS_PRECISE, format!("(r,c)=({},{}) mismatch", r, c));                
            }
        }  
    }

    #[test]
    fn test_ij_minors() {
        let data = [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]];
        let m4 = Matrix4f { data };
        let m00 = Matrix3f::ij_minor(0, 0, &m4);        
        let expected = [[6.0, 7.0, 8.0], [10.0, 11.0, 12.0], [14.0, 15.0, 16.0]];
        for r in 0..3 {
            for c in 0..3 {
                assert_f32_eq!(m00.data[r][c], expected[r][c], tests::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));                 
            }
        }  
    }

    #[test]
    
    fn test_cofactor_mat() {
        let data = [[1.0, 2.3, 12.2, 7.0], [-4.3, -233.1, 10.0, 2.8], [12.0, -12.0, 12.0, -13.0], [1.2, -1.4, 4.5, 0.0]];
        let m1 = Matrix4f { data, };

        let expected = [[-13558.5100000001,  296.670000000013,  3707.90000000008, -9366.71999999999],
                    [-96.1899999999441,  145.379999997094,  70.8800000000047, -157.559999999998],
                    [-7321.45399999992,  191.058000000077,  2011.82800000001, -2523.18299999997],
                    [ 56496.2600000000, -1636.85999999999, -22954.2100000001, 32472.8399999999]];

        let cof = m1.cofactor_mat();
        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(cof[r][c], expected[r][c], tests::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));              
            }
        }  
    }

    #[test]
    
    fn test_inverse_4x4_a() {
        let data = [[1.0, 2.3, 3.2, 7.0], [-4.3, -2.1, 10.0, 2.8], [4.2, -8.9, 0.04, 10.91], [1.2, -1.4, 1.5, 0.0]];
        let m1 = Matrix4f { data, };
        let inv_m1 = m1.inverse();

        let expected: [[f32; 4]; 4] = [[0.0893197126914255, -0.103083965680273, -0.0308526933946114, 0.497500455950639],
                        [0.125598521500081, -0.0397776211154350, -0.0703769304653849,- 0.000882653618195975],
                        [0.0457695165802688, 0.0453413928364795, -0.0410029803853367, 0.267842491862506],
                        [0.0679058906859326, 0.00706857673843424, 0.0462755958140106, -0.193223760941258]];

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(inv_m1[r][c], expected[r][c], tests::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));                
            }
        }                        
    }

    #[test]
    fn test_inverse_2x2() {        
        let mut m1 = Matrix2f::new();
        m1[0][0] = 17.0;
        m1[0][1] = 13.0;
        m1[1][0] = 2.0;
        m1[1][1] = 9.0;
        let inv_m1 = m1.inverse();

        let mut res = Matrix2f::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res[i][j] += m1[i][k] * inv_m1[k][j];
                }
            }    
        }        
        assert_f32_eq!(res[0][0], 1.0, tests::EPS_BAD); 
        assert_f32_eq!(res[0][1], 0.0, tests::EPS_BAD); 
        assert_f32_eq!(res[1][0], 0.0, tests::EPS_BAD); 
        assert_f32_eq!(res[1][1], 1.0, tests::EPS_BAD); 
    }
}
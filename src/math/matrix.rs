use std::ops::{Index, IndexMut, Mul};
use super::{
    Vector2f,
    Vector3f,
    Vector4f,
};
use std::f32;
use super::super::entities::Camera;

#[derive(Debug, Clone)]
pub struct Matrix4f {
    data: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Matrix4f {
            data,
        }
    }

    pub fn identity() -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            data[i][i] = 1.0;
        }
        Matrix4f {
            data,
        }
    }

    pub fn zeros() -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];
        for i in 0..4 {
            data[i][i] = 0.0;
        }
        Matrix4f {
            data,
        }
    }

    fn change_of_basis_3d(x: &Vector3f, y: &Vector3f, z: &Vector3f) -> Matrix4f {
        let mut data = [[0.0f32; 4]; 4];        
        for i in 0..3 {
            data[i][0] = x[i];        
            data[i][1] = y[i];        
            data[i][2] = z[i];
        }
        data[3][3] = 1.0;        
        Matrix4f {
            data,
        }
    }

    pub fn make_identity(&mut self) {        
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    pub fn create_gui_transform_matrix(translation: &Vector2f, scale: &Vector2f) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.scale(&Vector3f::new(scale.x, scale.y, 1.0));
        transform_mat.translate(&Vector3f::new(translation.x, translation.y, 0.0));        
        transform_mat
    }
    
    pub fn create_particle_transform_matrix(translation: &Vector3f, rotation_z_deg: f32, scale: f32, camera_pitch: f32, camera_yaw: f32) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.scale(&Vector3f::new(scale, scale, scale));
        // make sure particles always angle up with where the camera is in yaw and pitch
        // that way when we rotate them they will still face it. use signs opposite to the ones used to build create_view_matrix
        transform_mat.rotate(&Vector3f::new(-camera_pitch, camera_yaw, rotation_z_deg));
        transform_mat.translate(translation);
        transform_mat
    }

    pub fn create_transform_matrix(translation: &Vector3f, rot_xyz_degrees: &Vector3f, scale: f32) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.scale(&Vector3f::new(scale, scale, scale));
        transform_mat.rotate(rot_xyz_degrees);
        transform_mat.translate(translation);
        transform_mat
    }

    pub fn create_transform_matrix_with_s(translation: &Vector3f, rot_xyz_degrees: &Vector3f, scale: &Vector3f) -> Matrix4f {        
        let mut transform_mat = Matrix4f::identity();
        transform_mat.scale(scale);
        transform_mat.rotate(rot_xyz_degrees);
        transform_mat.translate(translation);
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

    pub fn update_ortho_projection_matrix(ortho_mat: &mut Matrix4f, frustum_width: f32, frustum_height: f32, frustum_length: f32) {        
        ortho_mat[0][0] = 2.0 / frustum_width;
        ortho_mat[1][1] = 2.0 / frustum_height;
        ortho_mat[2][2] = -2.0 / frustum_length;
        ortho_mat[3][3] = 1.0;
    }

    // view matrix makes objects move closer to the camera as we move towards them since it includes the negative of the camera translation
    // we dont want the skybox to move as we move around (but we do want it to rotate) so we zero out the translation
    pub fn create_skybox_view_matrix(camera: &Camera, skybox_rotation_deg: f32) -> Matrix4f {        
        // pitch is the rotation against transverse axis (pointing to right) -> for our object x axis is right
        // yaw is the rotation against vertical axis (pointing up) -> for our object y axis is up
        // roll is the rotation against longitudal axis (pointing forward) -> for our object z axis is forward
        let rotation_xyz_degrees = Vector3f::new(camera.pitch, -camera.yaw - skybox_rotation_deg, camera.roll);
        let mut view_mat = Matrix4f::identity();         
        view_mat.rotate(&rotation_xyz_degrees);
        view_mat
    }

    pub fn camera_change_of_basis(eye_position: &Vector3f, scene_center: &Vector3f, up: &Vector3f) -> Matrix4f {
        let mut forward = scene_center - eye_position;
        forward.normalize();
        let mut side = forward.cross_prod(&up);
        side.normalize();
        // up doesnt need to be perpendicular to forward -> so let's find the perp one
        let mut up = side.cross_prod(&forward);
        up.normalize();
        // our eye to scene center should be negative z
        // that means -foward is the positive which we use to do change of basis
        let forward = -forward;

        let change_basis = Matrix4f::change_of_basis_3d(&side, &up, &forward);
        change_basis
    }

    pub fn look_at(eye_position: &Vector3f, scene_center: &Vector3f, up: &Vector3f) -> Matrix4f {
        // preform change of basis
        let mut view_mat = Matrix4f::identity();
        let mut change_basis = Self::camera_change_of_basis(eye_position, scene_center, up); 
        // we want to find the components of vec in look at basis .. that means we want inverse of change of basis
        // it is orthonormal mat so transpose
        change_basis.transpose_ip();

        // first move to center of new basis
        view_mat.translate(&(-eye_position));
        // then find component of our vector in new basis
        view_mat.pre_multiply_in_place(&change_basis);
        
        view_mat
    }
    
    pub fn create_view_matrix(camera: &Camera) -> Matrix4f {
        Self::look_at(&camera.position, &camera.looking_at, &camera.up)
        // dont use this since it is restricted in the pitch angle altho it does work
        //Self::create_fps_view_matrix(&camera.position, -camera.pitch, camera.yaw)
    }

    // the allowed range for camera pitch (assuming pitch rot around X axis) is [-90, 90]
    // the allowed range for camera yaw (assuming yaw row around Y axis) is [0, 360]
    // the reason for ptich limit is due to yaw rotation. it would have to be reversed if you go out of the range
    // since rotation is measures in RHS as counter-clockwise from z to x and being upside down would mean the rotation would need to be clockwise
    pub fn create_fps_view_matrix(eye_position: &Vector3f, pitch: f32, yaw: f32) -> Matrix4f {
        let mut view_mat = Matrix4f::zeros();
        let (sa, ca) = pitch.to_radians().sin_cos();
        let (sb, cb) = yaw.to_radians().sin_cos();
        // the axis are rows because we are already dealing with the inverted camera transform matrix
        // the resulting view matrix is meant to turn everything around X axis by pitch and around Y by yaw
        // disregard the pitch and yaw terms ... this is more like the pitch yaw of an object alined around -z
        // V = M^-1 = (T*Ry*Rx)^-1
        let xaxis = Vector3f::new(cb, 0.0,-sb);
        let yaxis = Vector3f::new(sa*sb, ca, cb*sa);
        let zaxis = Vector3f::new(ca*sb, -sa, ca*cb);
        for i in 0..3 {
            view_mat[0][i] = xaxis[i];
            view_mat[0][3] = -(eye_position.dot_product(&xaxis));
            view_mat[1][i] = yaxis[i];
            view_mat[1][3] = -(eye_position.dot_product(&yaxis));
            view_mat[2][i] = zaxis[i];
            view_mat[2][3] = -(eye_position.dot_product(&zaxis));
        }
        view_mat[3][3] = 1.0;
        view_mat
    }

    pub fn create_view_matrix0(camera: &Camera) -> Matrix4f {
        let translation = &camera.position;
        // pitch is the rotation against transverse axis (pointing to right) -> for out object x axis is right
        // yaw is the rotation against vertical axis (pointing up) -> for our object y axis is up
        // roll is the rotation against longitudal axis (pointing forward) -> for our object z axis is forward

        // ok this is an incredible mess
        // the positive negative nonesense has to do i think with where the start point for rotations in 3d is
        // for example if you have a rotation about the y axis (the yaw in this case)
        // then 0 for that rotation means you are looking down the positive z axis (into the camera)
        // this is due to right hand rule and that the rotations start at the first axis 
        // since our camera correctly keeps track of pitch as being relative to z axis positive and since
        // we want our camera to rotate objects in the inverted direction we have to use -camera.yaw
        //
        // the matter is different for the pitch. we say our camera has positive pitch but really pitch is positive
        // if counter-clockwise and our camera is trying to turn things clockwise. so really our camera should say it has negative pitch
        // and then the inverted direction is the positive pitch which is what we use here
        let rotation_xyz_degrees = Vector3f::new(camera.pitch, -camera.yaw, camera.roll);        
        let mut view_mat = Matrix4f::identity();
        view_mat.translate(&(-translation)); 
        // the problem with inverse_rotate is that it is the transpose of tait bryan so the angles we specify with the 
        // inverse wont correspond to getting the final camera to have our desired pitch and yaw
        // to get our desired pitch and yaw we have to use tait-bryan or extrinsic euler angles and post multiply
        view_mat.rotate(&rotation_xyz_degrees);       
        view_mat
    }

    // translate using the translation matrix
    pub fn translate(&mut self, t: &Vector3f) {
        let tran_mat = Matrix4f{ data: [[1.0, 0.0, 0.0, t.x], [0.0, 1.0, 0.0, t.y], [0.0, 0.0, 1.0, t.z], [0.0, 0.0, 0.0, 1.0]] };
        self.pre_multiply_in_place(&tran_mat);
    }

    // scale using the scale matrix
    pub fn scale(&mut self, s: &Vector3f) {
        let scale_mat = Matrix4f { data: [[s.x, 0.0, 0.0, 0.0], [0.0, s.y, 0.0, 0.0], [0.0, 0.0, s.z, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        self.pre_multiply_in_place(&scale_mat);
    }

    pub fn get_rotation(roll: f32, pitch: f32, yaw: f32) -> Matrix4f {
        // tait-bryan rotation matrix -> HAS TO BE PRE-MULTIPLIED
        // will perform an intrinsic rotation about the fixed reference frame of the object
        // assumes a right-handed coordinate system (OpenGL) -> this affects the rotation matrix signs
        // the result after the rotation will be the object with the new yaw, pitch, roll in w.r.t its fixed reference frame
        let mut rot_mat = [[0.0f32; 4]; 4];
        rot_mat[3][3] = 1.0;
        let (sa, ca) = roll.to_radians().sin_cos();
        let (sb, cb) = pitch.to_radians().sin_cos();
        let (sc, cc) = yaw.to_radians().sin_cos();
        
        rot_mat[0][0] = cb*cc;
        rot_mat[0][1] = -cb*sc;
        rot_mat[0][2] = sb;

        rot_mat[1][0] = cc*sa*sb + ca*sc;
        rot_mat[1][1] = -sa*sb*sc + ca*cc;
        rot_mat[1][2] = -cb*sa;

        rot_mat[2][0] = -ca*cc*sb + sa*sc;
        rot_mat[2][1] = ca*sb*sc + cc*sa;
        rot_mat[2][2] = ca*cb;
        Matrix4f { 
            data: rot_mat 
        }
    }

    // the inverse rotation cancels this same rotation
    pub fn get_inverse_rotation(roll: f32, pitch: f32, yaw: f32) -> Matrix4f {
        let mut rot = Self::get_rotation(roll, pitch, yaw);
        rot.transpose_ip();
        rot
    }

    pub fn rotate(&mut self, rot_xyz_degrees: &Vector3f) {
        let rot_mat = Self::get_rotation(rot_xyz_degrees.x, rot_xyz_degrees.y, rot_xyz_degrees.z);
        self.pre_multiply_in_place(&rot_mat);
    }

    pub fn inverse_rotate(&mut self, rot_xyz_degrees: &Vector3f) {
        let rot_mat = Self::get_inverse_rotation(rot_xyz_degrees.x, rot_xyz_degrees.y, rot_xyz_degrees.z);
        self.pre_multiply_in_place(&rot_mat);
    }

    pub fn pre_multiply_in_place(&mut self, other: &Matrix4f) {
        let mut res_mat = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res_mat[i][j] += other.data[i][k] * self.data[k][j];
                }
            }
        }
        self.data = res_mat;
    }

    pub fn post_multiply_in_place(&mut self, other: &Matrix4f) {
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
        cofactor.transpose_ip();
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

    pub fn transpose_ip(&mut self) {
        for i in 0..3 {
            for j in (i+1)..4 {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[j][i];
                self.data[j][i] = temp;
            }
        }        
    }

    pub fn transpose(&self) -> Matrix4f {
        let mut res = self.clone();
        res.transpose_ip();
        res
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

impl Mul<&Matrix4f> for &Matrix4f {
    type Output = Matrix4f;

    fn mul(self, other: &Matrix4f) -> Matrix4f {
        let mut res = [[0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res[i][j] += self.data[i][k] * other.data[k][j];
                }
            }    
        }
        Matrix4f {
            data: res,
        }
    }
}

#[derive(Debug)]
pub struct Matrix3f {
    data: [[f32; 3]; 3],
}

impl Matrix3f {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Matrix3f {
            data,
        }
    }

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

    fn cofactor_mat(&self) -> Matrix3f {
        let mut data = [[0.0f32; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mat2 = Matrix2f::ij_minor(i, j, self);
                let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                data[i][j] = sign * mat2.determinant();
            }
        }
        Matrix3f {
            data,
        }
    }

    fn abjugate_mat(&self) -> Matrix3f {
        let mut cofactor = self.cofactor_mat();
        cofactor.transpose_ip();
        cofactor
    }

    pub fn transform(&self, vec: &Vector3f) -> Vector3f {
        let mut res = Vector3f::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            for j in 0..3 {
                res[i] += self.data[i][j] * vec[j];
            }
        }
        res
    }

    pub fn transpose_ip(&mut self) {
        for i in 0..2 {
            for j in (i+1)..3 {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[j][i];
                self.data[j][i] = temp;
            }
        }        
    }

    pub fn inverse(&self) -> Matrix3f {
        let mut abjugate = self.abjugate_mat();
        let determinant = self.determinant();
        let fac = 1.0 / determinant;
        for i in 0..3 {
            for j in 0..3 {
                abjugate.data[i][j] *= fac;
            }
        }
        abjugate
    }

    pub fn determinant(&self) -> f32 {      
        let minor_a = self.data[0][0] * (self.data[1][1]*self.data[2][2] - self.data[1][2]*self.data[2][1]);        
        let minor_b = self.data[0][1] * (self.data[1][0]*self.data[2][2] - self.data[1][2]*self.data[2][0]);
        let minor_c = self.data[0][2] * (self.data[1][0]*self.data[2][1] - self.data[1][1]*self.data[2][0]);        
        minor_a - minor_b + minor_c
    }
}

impl Index<usize> for Matrix3f {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &[f32; 3] {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix3f {

    fn index_mut(&mut self, index: usize) -> &mut [f32; 3] {
        &mut self.data[index]
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

    pub fn ij_minor(row: usize, col: usize, mat3: &Matrix3f) -> Matrix2f {
        let mut data = [[0.0f32; 2]; 2];
        let mut nc = 0;        
        for j in 0..3 {
            if j == col {
                continue
            }
            let mut nr = 0;
            for i in 0..3 {
                if i == row {
                    continue
                }
                data[nr][nc] = mat3.data[i][j];
                nr += 1;
            }                        
            nc += 1;
        }
        Matrix2f {
            data,
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn matrix_mul_4x4() {
        let m1 = Matrix4f {data: [[-1.3, 10.0, 2.8, 2.0], [-2.0, 3.0, -13.0, 9.0], [-1.4, 4.5, 0.0, 3.2], [-1.0, -2.0, 3.0, 5.0]] };
        let m2 = Matrix4f {data: [[-0.3, -7.0, 12.8, 2.0], [3.0, 3.0, 3.0, 3.0], [-4.0, -4.5, 4.0, 4.5], [-11.0, -22.0, 33.0, 55.0]] };        
        let result = &m1 * m2;
        
        let expected = Matrix4f {data: [[-2.81, -17.5, 90.560, 150.0], [-37.4, -116.5, 228.4, 441.5], [-21.28, -47.10, 101.18, 186.7], [-72.7, -122.50, 158.20, 280.5]] };  
        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(result[r][c], expected[r][c], test_constants::EPS_MEDIUM, format!("(r,c)=({},{}) mismatch", r, c));
            }
        }
    }

    #[test]
    fn translate_transformation() {
        let mut m1 = Matrix4f {data: [[-1.3, 10.0, 2.8, 2.0], [-2.0, 3.0, -13.0, 9.0], [-1.4, 4.5, 0.0, 3.2], [-1.0, -2.0, 3.0, 5.0]] };        
        let t = Vector3f::new(3.0, 2.4, -1.2);
                
        let mut trans_mat = Matrix4f::identity();
        for i in 0..3 {
            trans_mat[i][3] = t[i];
        }        
        let expected = &trans_mat * &m1;
        m1.translate(&t);

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(m1[r][c], expected[r][c], test_constants::EPS_MEDIUM, format!("(r,c)=({},{}) mismatch", r, c));
            }
        }
    }

    #[test]
    fn scale_transformation() {
        let mut m1 = Matrix4f {data: [[-1.3, 10.0, 2.8, 2.0], [-2.0, 3.0, -13.0, 9.0], [-1.4, 4.5, 0.0, 3.2], [-1.0, -2.0, 3.0, 5.0]] };        
        let s = Vector3f::new(3.0, 2.4, -1.2);
                
        let mut scale_mat = Matrix4f::identity();
        for i in 0..3 {
            scale_mat[i][i] = s[i];
        }
        let expected = &scale_mat * &m1;

        m1.scale(&s);

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(m1[r][c], expected[r][c], test_constants::EPS_MEDIUM, format!("(r,c)=({},{}) mismatch", r, c));
            }
        }
    }

    #[test]
    fn test_inverse_rotation() {
        let r = Matrix4f::get_rotation(30.0, -20.0, 150.0);
        let ri = Matrix4f::get_inverse_rotation(30.0, -20.0, 150.0);

        let res = &r * &ri;
        let expected = Matrix4f::identity();

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(res[r][c], expected[r][c], test_constants::EPS_MEDIUM, format!("(r,c)=({},{}) mismatch", r, c));
            }
        }
    }

    #[test]
    fn test_determinant_3x3_a() {
        let data = [[-233.1, 10.0, 2.8], [-12.0, 12.0, -13.0], [-1.4, 4.5, 0.0]];
        let m1 = Matrix3f { data, };
        assert_f32_eq!(-13558.51, m1.determinant(), test_constants::EPS_BAD);
    }

    #[test]    
    fn test_determinant_4x4_a() {
        let data = [[1.0, 2.3, 12.2, 7.0], [-4.3, -233.1, 10.0, 2.8], [12.0, -12.0, 12.0, -13.0], [1.2, -1.4, 4.5, 0.0]];
        let m1 = Matrix4f { data, };
        assert_f32_eq!(-33206.8290000003, m1.determinant(), test_constants::EPS_BAD);              
    }

    #[test]
    fn test_transpose_4x4_a() {
        let data = [[1.0, 2.3, 12.2, 7.0], [-4.3, -233.1, 10.0, 2.8], [12.0, -12.0, 12.0, -13.0], [1.2, -1.4, 4.5, 0.0]];
        let mut m1 = Matrix4f { data, };
        m1.transpose_ip();
        let expected = [[1.0, -4.3, 12.0, 1.2], [2.3, -233.1, -12.0, -1.4], [12.2, 10.0, 12.0, 4.5], [7.0, 2.8, -13.0, 0.0]];

        for r in 0..4 {
            for c in 0..4 {
                assert_f32_eq!(m1[r][c], expected[r][c], test_constants::EPS_PRECISE, format!("(r,c)=({},{}) mismatch", r, c));                
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
                assert_f32_eq!(m00.data[r][c], expected[r][c], test_constants::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));                 
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
                assert_f32_eq!(cof[r][c], expected[r][c], test_constants::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));              
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
                assert_f32_eq!(inv_m1[r][c], expected[r][c], test_constants::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));                
            }
        }                        
    }

    #[test]
    fn test_inverse_3x3_a() {
        let data = [[1.0, 2.3, 3.2], [-4.3, -2.1, 10.0], [4.2, -8.9, 0.04]];
        let m1 = Matrix3f { data, };
        let inv_m1 = m1.inverse();

        let expected: [[f32; 3]; 3] = [[0.264159553368453, -0.0848842363449036,  0.0882948167496337],
                        [0.125288324763309, -0.0398099106475468, -0.0705883191780382],
                        [0.139899156148730 , 0.0551396971357066,  0.0231432241749545]];

        for r in 0..3 {
            for c in 0..3 {
                assert_f32_eq!(inv_m1[r][c], expected[r][c], test_constants::EPS_BAD, format!("(r,c)=({},{}) mismatch", r, c));                
            }
        }                        
    }

    #[test]
    fn test_3x3_transform() {
        let m = Matrix3f::new([[1.0,1.0,2.0],[2.0,2.0,3.0],[1.0,1.0,1.0]]);
        let v = Vector3f::new(1.0,2.0,3.0);
        let mv = m.transform(&v);
        let expected = Vector3f::new(9.0,15.0,6.0);
        for r in 0..3 {
            assert_f32_eq!(mv[r], expected[r], test_constants::EPS_BAD, format!("(r)=({}) mismatch", r));
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
        assert_f32_eq!(res[0][0], 1.0, test_constants::EPS_BAD); 
        assert_f32_eq!(res[0][1], 0.0, test_constants::EPS_BAD); 
        assert_f32_eq!(res[1][0], 0.0, test_constants::EPS_BAD); 
        assert_f32_eq!(res[1][1], 1.0, test_constants::EPS_BAD); 
    }
}
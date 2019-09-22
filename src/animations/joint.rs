use crate::math::{
    Matrix4f,
    Vector3f,
    Quaternion,
};
use collada::Matrix4;

#[derive(Clone)]
pub struct Joint {
    pub index: usize,
    pub name: String,
    pub children: Vec<Joint>,
    // transform from model space with default joint config to model space with animated joint config
    pub animated_transform_model_space: Matrix4f,    
    // inverse transform to bind_matrix
    pub inverse_bind_matrix_model_space: Matrix4f,
}

impl Joint {
    pub fn new(index: usize, name: String, inverse_bind_matrix: Matrix4f) -> Self {
        Self {
            index,
            name,
            animated_transform_model_space: Matrix4f::identity(),
            inverse_bind_matrix_model_space: inverse_bind_matrix,
            children: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct JointTransform {
    pub position: Vector3f,
    pub rotation: Quaternion,
}

impl JointTransform {
    pub fn create_from_collada(column_mjr_mat: &Matrix4<f32>) -> Self {
        let mut row_major_rot_mat = Matrix4f::identity();
        let mut position = Vector3f::zero();
        for i in 0..3 {
            position[i] = column_mjr_mat[3][i];
            for j in 0..3 {
                row_major_rot_mat[i][j] = column_mjr_mat[j][i];
            }
        }   
        let rotation = Quaternion::from_rot_mat(&row_major_rot_mat);     
        JointTransform {
            position,
            rotation,
        }
    }
}
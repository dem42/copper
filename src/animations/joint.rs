use crate::constants::MAX_JOINTS;
use crate::math::{
    Matrix4f,
    Vector3f,
    Quaternion,
};
use std::collections::HashMap;
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

    pub fn apply_new_joint_poses(&mut self, cumulative_transform: &Matrix4f, poses: &HashMap<String, JointTransform>) {
        let jt = poses.get(&self.name);
        let joint_transform_os = if let Some(transform) = jt {
            transform.as_matrix()
        } else {
            Matrix4f::identity()
        };
        let joint_transform_os = joint_transform_os * cumulative_transform;
        self.animated_transform_model_space = &joint_transform_os * &self.inverse_bind_matrix_model_space;
        for ch_joint in self.children.iter_mut() {
            ch_joint.apply_new_joint_poses(&joint_transform_os, poses);
        }
    }

    pub fn collect_transforms(&self, accum: &mut AccumulatedJointTransforms) {
        accum.transforms[self.index].fill_from(&self.animated_transform_model_space);
        for ch_joint in self.children.iter() {
            ch_joint.collect_transforms(accum);
        }
    }
}

pub struct AccumulatedJointTransforms {
    pub transforms: [Matrix4f; MAX_JOINTS],
}

impl AccumulatedJointTransforms {
    pub fn new() -> Self {
        Self {
            transforms: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct JointTransform {
    // the position and rotation are with respect to the PARENT joint reference frame
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

    pub fn as_matrix(&self) -> Matrix4f {
        let mut rot_mat = self.rotation.as_rot_mat();
        rot_mat.translate(&self.position);
        rot_mat
    }

    pub fn interpolate(t1: &JointTransform, t2: &JointTransform, progression: f32) -> JointTransform {
        let intp_pos = Vector3f::lerp(&t1.position, &t2.position, progression);
        let intp_rot = Quaternion::slerp(&t1.rotation, &t2.rotation, progression);
        JointTransform {
            position: intp_pos,
            rotation: intp_rot,
        }
    }
}
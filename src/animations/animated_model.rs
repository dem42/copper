use crate::models::{
    RawModel,
    TextureId,
};

use super::joint::Joint;

pub struct AnimatedModel {
    // skin
    pub vao_id: RawModel,
    pub texture_id: TextureId,

    // skeleton
    pub root_joint: Joint,
    pub joint_cnt: usize,
}
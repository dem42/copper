use crate::models::{
    RawModel,
    TextureId,
};

use super::animation::Animation;
use super::joint::Joint;

#[derive(Clone)]
pub struct AnimatedModel {
    // skin
    pub raw_model: RawModel,
    pub tex_id: TextureId,

    // skeleton
    pub root_joint: Joint,
    pub joint_cnt: usize,

    pub animation: Animation,
}
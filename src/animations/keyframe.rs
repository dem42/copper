use super::joint::JointTransform;
use std::collections::HashMap;

pub struct Keyframe {
    pub timestamp: f32,
    pub pose_map: HashMap<String, JointTransform>,
}
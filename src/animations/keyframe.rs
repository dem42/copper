use super::joint::JointTransform;

pub struct Keyframe {
    pub timestamp: f32,
    pub pose: JointTransform,
}
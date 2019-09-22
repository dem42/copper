use super::joint::JointTransform;

#[derive(Clone)]
pub struct Keyframe {
    pub timestamp: f32,
    pub pose: JointTransform,
}
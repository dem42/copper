use super::joint::JointTransform;

#[derive(Clone, Debug)]
pub struct Keyframe {
    pub timestamp: f32,
    pub pose: JointTransform,
}
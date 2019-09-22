use super::keyframe::Keyframe;

#[derive(Clone)]
pub struct Animation {
    pub length_seconds: f32,
    pub joint_animations: Vec<JointAnimation>,
}

#[derive(Clone)]
pub struct JointAnimation {
    pub name: String,
    pub length_seconds: f32,
    pub keyframes: Vec<Keyframe>,
}
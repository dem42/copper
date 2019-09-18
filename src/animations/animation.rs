use super::keyframe::Keyframe;

pub struct Animation {
    pub length_seconds: f32,
    pub joint_animations: Vec<JointAnimation>,
}

pub struct JointAnimation {
    pub name: String,
    pub length_seconds: f32,
    pub keyframes: Vec<Keyframe>,
}
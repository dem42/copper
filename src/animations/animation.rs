use super::keyframe::Keyframe;

pub struct Animation {
    pub length_seconds: f32,
    pub keyframes: Vec<Keyframe>,
}
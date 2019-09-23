use super::keyframe::Keyframe;

#[derive(Clone)]
pub struct Animation {
    pub length_seconds: f32,
    pub joint_animations: Vec<JointAnimation>,
}

#[derive(Clone)]
pub struct JointAnimation {
    pub name: String,
    pub current_animation_time: f32,
    pub length_seconds: f32,
    pub keyframes: Vec<Keyframe>,
}

pub enum AnimationProgress<'a> {
    NotStarted,
    InProgress(&'a Keyframe, &'a Keyframe),
    Finished,
}

impl JointAnimation {
    pub fn get_keyframe_progress(&self) -> AnimationProgress {
        for i in 0..self.keyframes.len() {
            if self.current_animation_time < self.keyframes[i].timestamp {
                if i > 0 {
                    return AnimationProgress::InProgress(&self.keyframes[i-1], &self.keyframes[i]);
                } else {
                    return AnimationProgress::NotStarted;
                }
            }
        }
        AnimationProgress::Finished
    }
}
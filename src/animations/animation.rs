use super::keyframe::Keyframe;

#[derive(PartialEq, Hash, Clone)]
pub enum AnimationState {
    Playing,
    Stopped,
}

impl Default for AnimationState {
    fn default() -> AnimationState {
        AnimationState::Stopped
    }
}

#[derive(Clone, Default)]
pub struct Animation {
    pub length_seconds: f32,
    pub joint_animations: Vec<JointAnimation>,
    animation_state: AnimationState,
}

impl Animation {
    pub fn play(&mut self) {
        self.animation_state = AnimationState::Playing;
    }

    pub fn stop(&mut self) {
        self.animation_state = AnimationState::Stopped;
        for anim in self.joint_animations.iter_mut() {
            anim.current_animation_time = 0.0;
        }
    }

    pub fn is_playing(&self) -> bool {
        self.animation_state == AnimationState::Playing
    }
}

#[derive(Clone, Debug)]
pub struct JointAnimation {
    pub name: String,
    pub joint_name: String,
    pub current_animation_time: f32,
    pub length_seconds: f32,
    pub keyframes: Vec<Keyframe>,
}

pub enum AnimationProgress<'a> {
    NotStarted,
    InProgress(&'a Keyframe, &'a Keyframe),
    LastFrame(&'a Keyframe, &'a Keyframe),
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
        AnimationProgress::LastFrame(&self.keyframes[self.keyframes.len()-1], &self.keyframes[0])
    }

    pub fn get_joint_name(joint_transform_name: &str) -> Option<String> {
        let byte_offset_suffix = joint_transform_name.find("/transform");
        byte_offset_suffix.map(|byte_offset| {
            let (prefix, _) = joint_transform_name.split_at(byte_offset);
            String::from(prefix)
        })
    }
}
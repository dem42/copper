use crate::entities::AnimatedEntity;
use crate::display::Display;
use super::animation::*;
use super::joint::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Animator;

impl Animator {
    pub fn update_animation(&self, animated_entity: &mut AnimatedEntity, display: &Display) {
        let animation = &mut animated_entity.model.animation;
        let mut joint_poses = HashMap::new();
        let frame_time = display.frame_time_sec;
        for joint_animation in animation.joint_animations.iter_mut() {
            joint_animation.current_animation_time += frame_time;
            let progress = joint_animation.get_keyframe_progress();            
            match progress {
                AnimationProgress::InProgress(k1, k2) => {
                    let dt = Self::calculate_progress_time(joint_animation.current_animation_time, k1.timestamp, k2.timestamp);
                    let pose = JointTransform::interpolate(&k1.pose, &k2.pose, dt);
                    joint_poses.insert(joint_animation.name.clone(), pose);
                },
                AnimationProgress::NotStarted => {},
                AnimationProgress::Finished => {
                    // loop animation
                    joint_animation.current_animation_time = joint_animation.current_animation_time % joint_animation.length_seconds;
                },
            }
        }

        animated_entity.model.root_joint.apply_new_joint_poses(&crate::math::Matrix4f::identity(), &joint_poses);
    }

    fn calculate_progress_time(cur_time: f32, t1: f32, t2: f32) -> f32 {
        (cur_time - t1) / (t2 - t1)
    }
}
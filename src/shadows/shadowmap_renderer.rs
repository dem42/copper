use crate::entities::{
    Camera,
    Entity,
    Light,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};
use super::shadow_box::ShadowBox;

pub struct ShadowMapRenderer {
    shadow_box: ShadowBox,
    world_to_lightspace: Matrix4f,
    ortho_proj_mat: Matrix4f,
}

impl ShadowMapRenderer {

    pub fn new(aspect_ratio: f32) -> Self {
        let shadow_box = ShadowBox::new(aspect_ratio);
        let world_to_lightspace = Matrix4f::identity();
        let ortho_proj_mat = Matrix4f::create_ortho_projection_matrix(shadow_box.width(), shadow_box.height(), shadow_box.length());
        ShadowMapRenderer {
            shadow_box,
            world_to_lightspace,
            ortho_proj_mat,
        }
    }

    pub fn prepare(&mut self, camera: &Camera, sun: &Light) {

    }

    pub fn render(&mut self, entities: &Vec<Entity>) {

    }

    pub fn cleanup(&mut self) {

    }

    fn update_world_to_lightspace(&mut self, light_direction: Vector3f, center: Vector3f) {        
        let pitch = light_direction.y.atan2(light_direction.z);
        let yaw = light_direction.x.atan2(light_direction.z);
        let mut rot = Matrix4f::calculate_rotation_from_rpy(0.0, pitch.to_degrees(), yaw.to_degrees());
        rot[0][3] = center.x;
        rot[0][3] = center.y;
        rot[0][3] = center.z;
        rot[3][3] = 1.0;
    }

}
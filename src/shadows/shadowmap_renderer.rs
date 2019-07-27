use crate::entities::{
    Camera,
    Entity,
    Light,
    Player,
    Terrain,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};
use super::shadow_box::ShadowBox;
use super::shadow_shader::ShadowShader;

pub struct ShadowMapRenderer {
    shadow_shader: ShadowShader,
    shadow_box: ShadowBox,
    world_to_lightspace: Matrix4f,
    ortho_proj_mat: Matrix4f,
    bias: Matrix4f,
    mvp_matrix: Matrix4f,
}

impl ShadowMapRenderer {

    pub fn new(aspect_ratio: f32) -> Self {
        let shadow_box = ShadowBox::new(aspect_ratio);
        let world_to_lightspace = Matrix4f::identity();
        let ortho_proj_mat = Matrix4f::identity();
        let bias = ShadowMapRenderer::create_bias_matrix();
        let shadow_shader = ShadowShader::new();
        let mvp_matrix = Matrix4f::identity();
        ShadowMapRenderer {
            shadow_shader,
            shadow_box,
            world_to_lightspace,
            ortho_proj_mat,
            bias,
            mvp_matrix,
        }
    }

    pub fn start_render(&mut self, camera: &Camera, sun: &Light) {
        self.update_world_to_lightspace(-(&sun.position), &camera.position);
        self.shadow_box.update(camera, &self.world_to_lightspace);
        Matrix4f::update_ortho_projection_matrix(&mut self.ortho_proj_mat, self.shadow_box.width(), self.shadow_box.height(), self.shadow_box.length());
        self.shadow_shader.start();

        self.mvp_matrix.make_identity();
        self.mvp_matrix.multiply_in_place(&self.ortho_proj_mat);
        self.mvp_matrix.multiply_in_place(&self.world_to_lightspace);
        self.shadow_shader.load_mvp_matrix(&self.mvp_matrix);
    }

    pub fn render(&mut self, entities: &Vec<Entity>) {

    }

    pub fn render_terrain(&mut self, terrain: &Vec<Terrain>) {

    }

    pub fn render_player(&mut self, player: &Player) {

    }

    pub fn stop_render(&mut self) {
        self.shadow_shader.stop();
    }

    fn update_world_to_lightspace(&mut self, light_direction: Vector3f, center: &Vector3f) {
        self.world_to_lightspace.make_identity();
        let yaw = light_direction.x.atan2(light_direction.z);
        let pitch = (light_direction.y / light_direction.length()).asin();
        let angles = Vector3f {x: pitch.to_degrees(), y: yaw.to_degrees(), z: 0.0};
        self.world_to_lightspace.rotate_tait_bryan_xyz(&angles);
        self.world_to_lightspace.translate(&(-center));
    }

    // we want to use the lightspace transform in a shader to sample from the depth map
    // the projection to lightspace ndc coords will leave us in the unit cube [-1,1]
    // but a texture has coords in range [0,1] so we use the bias matrix to apply the conversion directly to the matrix
    fn create_bias_matrix() -> Matrix4f {
        let mut bias = Matrix4f::identity();
        let s = Vector3f::new(0.5, 0.5, 0.5);
        bias.scale(&s);
        bias.translate(&s);
        bias
    }

}
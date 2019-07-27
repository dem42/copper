use crate::entities::{
    Camera,
    Entity,
    Light,
};
use crate::gl;
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::models::{
    RawModel,
    TexturedModel,
};
use super::shadow_box::ShadowBox;
use super::shadow_shader::ShadowShader;

pub struct ShadowMapRenderer {
    shadow_shader: ShadowShader,
    shadow_box: ShadowBox,
    world_to_lightspace: Matrix4f,
    ortho_proj_mat: Matrix4f,
    bias: Matrix4f,
    vp_matrix: Matrix4f,
    mvp_matrix: Matrix4f,
}

impl ShadowMapRenderer {

    pub fn new(aspect_ratio: f32) -> Self {
        let shadow_box = ShadowBox::new(aspect_ratio);
        let world_to_lightspace = Matrix4f::identity();
        let ortho_proj_mat = Matrix4f::identity();
        let bias = ShadowMapRenderer::create_bias_matrix();
        let shadow_shader = ShadowShader::new();
        let vp_matrix = Matrix4f::identity();
        let mvp_matrix = Matrix4f::identity();
        ShadowMapRenderer {
            shadow_shader,
            shadow_box,
            world_to_lightspace,
            ortho_proj_mat,
            bias,
            vp_matrix,
            mvp_matrix,
        }
    }

    pub fn start_render(&mut self, camera: &Camera, sun: &Light) {
        self.update_world_to_lightspace(-(&sun.position), &camera.position);
        self.shadow_box.update(camera, &self.world_to_lightspace);
        Matrix4f::update_ortho_projection_matrix(&mut self.ortho_proj_mat, self.shadow_box.width(), self.shadow_box.height(), self.shadow_box.length());
        self.shadow_shader.start();

        self.vp_matrix.make_identity();
        self.vp_matrix.multiply_in_place(&self.ortho_proj_mat);
        self.vp_matrix.multiply_in_place(&self.world_to_lightspace);        
    }

    pub fn prepare_textured_model(&mut self, model: &TexturedModel) {
        gl::bind_vertex_array(model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
    }

    pub fn render(&mut self, entities: &Vec<&Entity>) {        
        for entity in entities.iter() {      
            self.render_entity(entity);
        }
    }

    pub fn render_entity(&mut self, entity: &Entity) {
        self.mvp_matrix.make_identity();
        self.mvp_matrix.multiply_in_place(&self.vp_matrix);
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        self.mvp_matrix.multiply_in_place(&transform_mat);
        self.shadow_shader.load_mvp_matrix(&self.mvp_matrix);

        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);            
    }

    pub fn cleanup_textured_model(&mut self) {
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);
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
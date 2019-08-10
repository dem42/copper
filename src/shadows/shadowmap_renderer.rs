use crate::display::Display;
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
    pub shadow_box: ShadowBox,
    world_to_lightspace: Matrix4f,
    ortho_proj_mat: Matrix4f,
    bias: Matrix4f,
    vp_matrix: Matrix4f,
    mvp_matrix: Matrix4f,
    //test_proj_matrix: Matrix4f,
}

impl ShadowMapRenderer {

    pub fn new(aspect_ratio: f32) -> Self {
        let shadow_box = ShadowBox::new(aspect_ratio, Display::FOV_HORIZONTAL, Display::NEAR, -ShadowBox::SHADOW_DISTANCE);
        let world_to_lightspace = Matrix4f::identity();
        let ortho_proj_mat = Matrix4f::identity();
        let bias = ShadowMapRenderer::create_bias_matrix();
        let shadow_shader = ShadowShader::new();
        let vp_matrix = Matrix4f::identity();
        let mvp_matrix = Matrix4f::identity();
        //let proj_mat = Matrix4f::create_projection_matrix(-50.0, -100.0, Display::FOV_HORIZONTAL, aspect_ratio);
        ShadowMapRenderer {
            shadow_shader,
            shadow_box,
            world_to_lightspace,
            ortho_proj_mat,
            bias,
            vp_matrix,
            mvp_matrix,
            //test_proj_matrix: proj_mat,
        }
    }

    pub fn start_render(&mut self, camera: &Camera, sun: &Light) {
        let (light_pitch_dg, light_yaw_dg) = ShadowMapRenderer::calc_light_pitch_yaw_dg(&sun.position);
        // testing with thinmatrix impl
        // self.shadow_box.update(camera, light_pitch_dg, light_yaw_dg);
        self.shadow_box.update_odd(camera, &self.world_to_lightspace);
        self.update_world_to_lightspace(light_pitch_dg, light_yaw_dg);
        Matrix4f::update_ortho_projection_matrix(&mut self.ortho_proj_mat, self.shadow_box.width, self.shadow_box.height, self.shadow_box.length);

        gl::enable(gl::DEPTH_TEST);
        gl::clear(gl::DEPTH_BUFFER_BIT);
        self.shadow_shader.start();

        self.vp_matrix.make_identity();
        self.vp_matrix.multiply_in_place(&self.ortho_proj_mat);
        self.vp_matrix.multiply_in_place(&self.world_to_lightspace);
        // self.vp_matrix.multiply_in_place(&self.test_proj_matrix);
        // let cam_view = Matrix4f::create_view_matrix(camera);
        // self.vp_matrix.multiply_in_place(&cam_view);
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

    pub fn get_to_shadow(&self) -> Matrix4f {
        let mut res = Matrix4f::identity();
        res.multiply_in_place(&self.bias);
        res.multiply_in_place(&self.ortho_proj_mat);
        res.multiply_in_place(&self.world_to_lightspace);
        res
    }

    fn calc_light_pitch_yaw_dg(to_light_direction: &Vector3f) -> (f32, f32) {
        let yaw = (-to_light_direction.x).atan2(-to_light_direction.z);
        let pitch = (-to_light_direction.y / to_light_direction.length()).asin();
        (pitch.to_degrees(), yaw.to_degrees())
    }

    fn update_world_to_lightspace(&mut self, pitch: f32, yaw: f32) {
        self.world_to_lightspace.make_identity();        
        let angles = Vector3f::new(pitch, -yaw, 0.0);
        self.world_to_lightspace.rotate_tait_bryan_zyx(&angles);
        let center = &self.shadow_box.world_space_center;// + Vector3f::new(0.0, 0.0, -2.0*ShadowBox::OFFSET);
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
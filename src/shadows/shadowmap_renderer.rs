use crate::display::Display;
use crate::entities::{
    Camera,
    Entity,
    Light,
    Terrain,
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
    bias: Matrix4f,
    vp_matrix: Matrix4f,
    mvp_matrix: Matrix4f,
}

impl ShadowMapRenderer {

    pub fn new(aspect_ratio: f32) -> Self {
        let shadow_box = ShadowBox::new(aspect_ratio, Display::FOV_HORIZONTAL, Display::NEAR, -ShadowBox::SHADOW_DISTANCE);
        let world_to_lightspace = Matrix4f::identity();        
        let bias = ShadowMapRenderer::create_bias_matrix();
        let shadow_shader = ShadowShader::new();
        let vp_matrix = Matrix4f::identity();
        let mvp_matrix = Matrix4f::identity();
        ShadowMapRenderer {
            shadow_shader,
            shadow_box,
            world_to_lightspace,            
            bias,
            vp_matrix,
            mvp_matrix,
        }
    }

    pub fn start_render(&mut self, camera: &Camera, sun: &Light) {                
        let (pitch, yaw) = Self::calc_light_pitch_yaw_dg(&sun.position);
        let world_to_lightspace_non_moving = Matrix4f::create_fps_view_matrix(&Vector3f::ZERO, pitch, yaw);
        self.shadow_box.update(camera, &world_to_lightspace_non_moving);        
        self.update_world_to_lightspace(pitch, yaw);
        
        gl::enable(gl::DEPTH_TEST);
        gl::clear(gl::DEPTH_BUFFER_BIT);
        self.shadow_shader.start();

        self.vp_matrix.make_identity();
        self.vp_matrix.pre_multiply_in_place(&self.world_to_lightspace);
        self.vp_matrix.pre_multiply_in_place(&self.shadow_box.ortho_proj_mat);
    }

    pub fn prepare_textured_model(&mut self, model: &TexturedModel) {
        gl::active_texture(gl::TEXTURE0);        
        gl::bind_texture(gl::TEXTURE_2D, model.texture.tex_id);
        gl::bind_vertex_array(model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);        
    }

    pub fn render(&mut self, entities: &Vec<&Entity>) {        
        for entity in entities.iter() {      
            self.render_entity(entity);
        }
    }

    pub fn render_entity(&mut self, entity: &Entity) {
        self.mvp_matrix.make_identity();
        self.mvp_matrix.post_multiply_in_place(&self.vp_matrix);
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        self.mvp_matrix.post_multiply_in_place(&transform_mat);
        self.shadow_shader.load_mvp_matrix(&self.mvp_matrix);

        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);            
    }

    pub fn cleanup_textured_model(&mut self) {        
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::bind_vertex_array(0);
    }

    pub fn render_terrain(&mut self, terrains: &Vec<Terrain>) {
        for terrain in terrains.iter() {
            gl::bind_vertex_array(terrain.model.raw_model.vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);            
            
            let terrain_pos = Vector3f::new(terrain.x as f32, 0.0, terrain.z as f32);
            let terrain_rot = Vector3f::new(0.0, 0.0, 0.0);
            let transform_mat = Matrix4f::create_transform_matrix(&terrain_pos, &terrain_rot, 1.0);

            self.mvp_matrix.make_identity();
            self.mvp_matrix.pre_multiply_in_place(&transform_mat);
            self.mvp_matrix.pre_multiply_in_place(&self.vp_matrix);

            self.shadow_shader.load_mvp_matrix(&self.mvp_matrix);
            gl::draw_elements(gl::TRIANGLES, terrain.model.raw_model.vertex_count, gl::UNSIGNED_INT);

            gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);            
        }
        gl::bind_vertex_array(0);
    }

    pub fn stop_render(&mut self) {
        self.shadow_shader.stop();
    }

    pub fn get_to_shadow(&self) -> Matrix4f {
        let mut res = Matrix4f::identity();
        res.pre_multiply_in_place(&self.world_to_lightspace);
        res.pre_multiply_in_place(&self.shadow_box.ortho_proj_mat);
        res.pre_multiply_in_place(&self.bias);
        res
    }

    fn calc_light_pitch_yaw_dg(to_light_direction: &Vector3f) -> (f32, f32) {
        let yaw = (-to_light_direction.x).atan2(-to_light_direction.z);
        let pitch = (-to_light_direction.y / to_light_direction.length()).asin();
        (pitch.to_degrees(), yaw.to_degrees())
    }

    fn update_world_to_lightspace(&mut self, pitch: f32, yaw: f32) {
        // this is a different to_lightspace matrix -> namely this one is moving whilst the one we compute in start_render is static
        // the reason this is important is that this allows us to remove the circular dependency between computing the center of the shadowbox
        // and using the center to compute the to_lightspace matrix
        // if the dependency is there then the frustum bounding box (shadow box) jumps around too much which seems to causes it to not correctly center on the player        
        let center = &self.shadow_box.world_space_center;
        self.world_to_lightspace = Matrix4f::create_fps_view_matrix(center, pitch, yaw);
    }
    // we want to use the lightspace transform in a shader to sample from the depth map
    // the projection to lightspace ndc coords will leave us in the unit cube [-1,1]
    // but a texture has coords in range [0,1] so we use the bias matrix to apply the conversion directly to the matrix
    fn create_bias_matrix() -> Matrix4f {
        let mut bias = Matrix4f::identity();
        let s = Vector3f::new(0.5, 0.5, 0.5);
        let t = Vector3f::new(0.5, 0.5, 0.5);
        bias.scale(&s);
        bias.translate(&t);
        bias
    }
}
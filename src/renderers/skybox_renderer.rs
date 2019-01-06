use crate::entities::{
    Camera,
};
use crate::gl;
use crate::math::{
    Matrix4f,
};
use crate::models::{
    SkyboxModel,
    RawModel,
};
use crate::shaders::SkyboxShader;

pub struct SkyboxRenderer {
    shader: SkyboxShader,    
}

impl SkyboxRenderer {
    pub fn new(proj_matrix: &Matrix4f) -> SkyboxRenderer {
        let mut skybox_shader = SkyboxShader::new();
        skybox_shader.start();
        skybox_shader.load_projection_matrix(proj_matrix);
        skybox_shader.stop();
        SkyboxRenderer {
            shader: skybox_shader,
        }
    }

    pub fn render(&mut self, camera: &Camera, skybox_model: &SkyboxModel) {
        self.shader.start();        
        self.shader.load_view_matrix(camera);

        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(skybox_model.texture_id, gl::TEXTURE_CUBE_MAP);

        gl::bind_vertex_array(skybox_model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::draw_arrays(gl::TRIANGLES, 0, skybox_model.raw_model.vertex_count);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);

        gl::bind_texture(0, gl::TEXTURE_CUBE_MAP);

        self.shader.stop();
    }
}
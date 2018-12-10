use crate::gl;
use crate::entities::{
    Entity,
    Camera,
    Light,
};
use crate::shaders::StaticShader;
use crate::math::Matrix4f;
use crate::loader::{
    TexturedModel,
    RawModel,
};

pub struct EntityRenderer {
    shader: StaticShader,
}

impl EntityRenderer {    
    
    pub fn new(projection_matrix: &Matrix4f) -> EntityRenderer {     
        let mut shader = StaticShader::new();
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.stop();
        EntityRenderer {
            shader,
        }
    }

    pub fn start_render(&mut self, light: &Light, camera: &Camera) {
        self.shader.start();
        self.shader.load_light(light);
        self.shader.load_view_matrix(camera);  
    }

    pub fn stop_render(&mut self) {        
        self.shader.stop();
    }

    pub fn prepare_textured_model(&mut self, textured_model: &TexturedModel) {
        gl::bind_vertex_array(textured_model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        self.shader.load_shine_variables(textured_model.texture.shine_damper, textured_model.texture.reflectivity);

        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(textured_model.texture.tex_id, gl::TEXTURE_2D);
    }

    pub fn render(&mut self, entity: &Entity) {        
        // load transform matrix into shader
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        self.shader.load_transformation_matrix(&transform_mat);
        
        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);
    }

    pub fn unprepare_textured_model(&self) {
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        gl::bind_vertex_array(0);
        gl::bind_texture(0, gl::TEXTURE_2D);
    }
}
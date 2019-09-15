use crate::gl;
use crate::entities::{
    Entity,
    Camera,
};
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,
};
use crate::models::{
    TexturedModel,
    RawModel,
};
use crate::shaders::EnvMapShader;

pub struct EnvMapRenderer {
    shader: EnvMapShader,
    proj_mat: Matrix4f,
}

impl EnvMapRenderer {    
    
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        let shader = EnvMapShader::new();        
        Self {
            shader,
            proj_mat: projection_matrix.clone(),
        }
    }
   
    pub fn render(&mut self, entities: &Vec<Entity>, camera: &Camera) {
        for entity in entities {
            self.prepare_textured_model(&entity.model);
            self.render_entity(entity, camera);
        }
    }

    fn prepare_textured_model(&mut self, textured_model: &TexturedModel) {
        gl::bind_vertex_array(textured_model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(gl::TEXTURE_2D, textured_model.texture.tex_id.unwrap());
    }

    fn render_entity(&mut self, entity: &Entity, camera: &Camera) {
        self.shader.start();

        self.prepare_textured_model(&entity.model);

        // load transform matrix into shader
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        let view_mat = Matrix4f::create_view_matrix(camera);
        let mvp = transform_mat * (view_mat * &self.proj_mat);
        self.shader.load_mvp_matrix(&mvp);
                
        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);

        self.shader.stop();
    }

    pub fn unprepare_textured_model(&self, textured_model: &TexturedModel) {
        if textured_model.texture.has_transparency {
            gl::helper::enable_backface_culling(); // restore backbace culling for next model
        }
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        gl::bind_vertex_array(0);
        gl::bind_texture(gl::TEXTURE_2D, 0);
    }
}
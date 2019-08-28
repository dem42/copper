use crate::gl;
use crate::entities::{
    Entity,
    Camera,
    Light,
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
use crate::shaders::StaticShader;
use crate::shadows::shadow_params::ShadowParams;

pub struct EntityRenderer {
    shader: StaticShader,
}

impl EntityRenderer {    
    
    pub fn new(projection_matrix: &Matrix4f) -> EntityRenderer {     
        let mut shader = StaticShader::new();
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.connect_texture_units();
        shader.stop();
        EntityRenderer {
            shader,
        }
    }
    
    pub fn start_render(&mut self, lights: &Vec<Light>, camera: &Camera, sky_color: &Vector3f, to_shadow_space: &Matrix4f, shadow_params: &ShadowParams) {
        self.shader.start();
        self.shader.load_lights(lights);
        self.shader.load_view_matrix(camera);
        self.shader.load_sky_color(sky_color);
        
        self.shader.load_to_shadowmap_space(to_shadow_space);
        self.shader.load_shadow_params(shadow_params);

        gl::active_texture(gl::TEXTURE1);
        gl::bind_texture(gl::TEXTURE_2D, shadow_params.shadow_map_texture);
    }

    pub fn stop_render(&mut self) {
        self.shader.stop();
    }

    pub fn prepare_textured_model(&mut self, textured_model: &TexturedModel, clip_plane: &Vector4f) {
        if textured_model.texture.has_transparency {
            gl::helper::disable_culling();
        }

        gl::bind_vertex_array(textured_model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        self.shader.load_shine_variables(textured_model.texture.shine_damper, textured_model.texture.reflectivity);
        self.shader.load_uses_fake_lighting(textured_model.texture.uses_fake_lighting);
        self.shader.load_atlas_number_of_rows(textured_model.texture.number_of_rows_in_atlas);

        // clip plane for water 
        self.shader.load_clip_plane(clip_plane);

        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(gl::TEXTURE_2D, textured_model.texture.tex_id);
    }

    pub fn render(&mut self, entity: &Entity) {
        // load transform matrix into shader
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        self.shader.load_transformation_matrix(&transform_mat);
        self.shader.load_atlas_offset(&entity.get_atlas_offset());
        
        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);
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
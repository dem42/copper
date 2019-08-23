use crate::gl;
use crate::entities::{
    Camera,
    Light,
    Terrain,
};
use crate::shaders::TerrainShader;
use crate::shadows::shadow_box::ShadowBox;
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,    
};
use crate::models::{
    RawModel,
};

pub struct TerrainRenderer {
    shader: TerrainShader,
}

impl TerrainRenderer {    
    
    pub fn new(projection_matrix: &Matrix4f) -> TerrainRenderer {     
        let mut shader = TerrainShader::new();
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.connect_texture_units();
        shader.stop();
        TerrainRenderer {
            shader,
        }
    }

    pub fn start_render(&mut self, lights: &Vec<Light>, camera: &Camera, sky_color: &Vector3f, to_shadowmap_space: Matrix4f, shadow_map_texture: u32) {
        self.shader.start();
        // we do this more than once because we may want to change the light, view, sky color
        // but we do them once per model type, because the type has one shader
        self.shader.load_lights(lights);
        self.shader.load_view_matrix(camera);  
        self.shader.load_sky_color(sky_color);
        self.shader.load_to_shadowmap_space(&to_shadowmap_space);
        self.shader.load_shadow_distance(ShadowBox::SHADOW_DISTANCE);

        gl::active_texture(gl::TEXTURE5);
        gl::bind_texture(gl::TEXTURE_2D, shadow_map_texture);
    }

    pub fn stop_render(&mut self) {          
        self.shader.stop();
    }

    pub fn prepare_terrain(&mut self, terrain: &Terrain, clip_plane: &Vector4f) {
        gl::bind_vertex_array(terrain.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        self.shader.load_shine_variables(1.0, 0.0);
        self.shader.load_clip_plane(clip_plane);

        // configure texture units
        gl::active_texture(gl::TEXTURE0); 
        gl::bind_texture(gl::TEXTURE_2D, terrain.texture_pack.background_texture.tex_id);
        gl::active_texture(gl::TEXTURE1); 
        gl::bind_texture(gl::TEXTURE_2D, terrain.texture_pack.r_texture.tex_id);
        gl::active_texture(gl::TEXTURE2); 
        gl::bind_texture(gl::TEXTURE_2D, terrain.texture_pack.g_texture.tex_id);
        gl::active_texture(gl::TEXTURE3); 
        gl::bind_texture(gl::TEXTURE_2D, terrain.texture_pack.b_texture.tex_id);
        gl::active_texture(gl::TEXTURE4); 
        gl::bind_texture(gl::TEXTURE_2D, terrain.blend_texture.tex_id);
    }

    pub fn render(&mut self, terrain: &Terrain) {        
        // load transform matrix into shader
        let terrain_pos = Vector3f::new(terrain.x as f32, 0.0, terrain.z as f32);
        let terrain_rot = Vector3f::new(0.0, 0.0, 0.0);
        let transform_mat = Matrix4f::create_transform_matrix(&terrain_pos, &terrain_rot, 1.0);
        self.shader.load_transformation_matrix(&transform_mat);
        
        gl::draw_elements(gl::TRIANGLES, terrain.model.raw_model.vertex_count, gl::UNSIGNED_INT);
    }

    pub fn unprepare_terrain(&self) {
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        gl::bind_vertex_array(0);
        gl::bind_texture(gl::TEXTURE_2D, 0);
    }
}
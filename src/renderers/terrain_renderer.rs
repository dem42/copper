use crate::gl;
use crate::entities::{
    Camera,
    Light,
    Terrain,
};
use crate::shaders::TerrainShader;
use crate::math::{
    Vector3f,
    Matrix4f
};
use crate::loader::{
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
        shader.stop();
        TerrainRenderer {
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

    pub fn prepare_terrain(&mut self, terrain: &Terrain) {
        gl::bind_vertex_array(terrain.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        self.shader.load_shine_variables(terrain.texture.shine_damper, terrain.texture.reflectivity);

        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(terrain.texture.tex_id, gl::TEXTURE_2D);
    }

    pub fn render(&mut self, terrain: &Terrain) {        
        // load transform matrix into shader
        let terrain_pos = Vector3f::new(terrain.x as f32, 0.0, terrain.z as f32);
        let terrain_rot = Vector3f::new(0.0, 0.0, 0.0);
        let transform_mat = Matrix4f::create_transform_matrix(&terrain_pos, &terrain_rot, 1.0);
        self.shader.load_transformation_matrix(&transform_mat);
        
        gl::draw_elements(gl::TRIANGLES, terrain.raw_model.vertex_count, gl::UNSIGNED_INT);
    }

    pub fn unprepare_terrain(&self) {
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);

        gl::bind_vertex_array(0);
        gl::bind_texture(0, gl::TEXTURE_2D);
    }
}
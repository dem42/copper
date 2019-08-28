use super::shader_program::ShaderProgram;
use crate::entities::{
    Camera,
    Light,
};
use crate::models::RawModel;
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,
};

const NUM_LIGHTS: usize = 4;

pub struct TerrainShader {
    program: ShaderProgram,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
    location_light_pos: [i32; NUM_LIGHTS],
    location_light_color: [i32; NUM_LIGHTS],
    location_shine_damper: i32,
    location_reflectivity: i32,
    location_sky_color: i32,
    location_background_sampler: i32,
    location_r_sampler: i32,
    location_g_sampler: i32,
    location_b_sampler: i32,
    location_blend_map_sampler: i32,
    location_attenuation: [i32; NUM_LIGHTS],
    location_clip_plane: i32,
    location_to_shadowmap_space: i32,
    location_shadowmap: i32,
    location_shadow_distance: i32,
    location_shadow_map_size: i32,
}

impl TerrainShader {
    pub fn new() -> TerrainShader {
        let (
            mut location_transformation_matrix, 
            mut location_projection_matrix,
            mut location_view_matrix,
            mut location_light_pos,
            mut location_light_color,
            mut location_shine_damper,
            mut location_reflectivity,
            mut location_sky_color,
        ) = Default::default();

        let (
            mut location_background_sampler,
            mut location_r_sampler,
            mut location_g_sampler,
            mut location_b_sampler,
            mut location_blend_map_sampler,
            mut location_attenuation,
            mut location_clip_plane,            
        ) = Default::default();

        let (
            mut location_to_shadowmap_space,
            mut location_shadowmap,
            mut location_shadow_distance,
            mut location_shadow_map_size,
        ) = Default::default();
        
        let shader_program = ShaderProgram::new(
            "res/shaders/terrainVertexShader.glsl", 
            "res/shaders/terrainFragShader.glsl", 
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "tex_coord");
                shader_prog.bind_attribute(RawModel::NORMAL_ATTRIB, "normal");
            },
            |shader_prog| {                
                location_transformation_matrix = shader_prog.get_uniform_location("transform");
                location_projection_matrix = shader_prog.get_uniform_location("projection_matrix");
                location_view_matrix = shader_prog.get_uniform_location("view_matrix");
                // diffuse lighting
                location_light_pos = [0i32; NUM_LIGHTS];
                location_light_color = [0i32; NUM_LIGHTS];
                for i in 0..NUM_LIGHTS {
                    location_light_pos[i] = shader_prog.get_uniform_location(&format!("light_pos[{}]", i));
                    location_light_color[i] = shader_prog.get_uniform_location(&format!("light_color[{}]", i));
                }
                // specular lighting
                location_shine_damper = shader_prog.get_uniform_location("shine_damper");
                location_reflectivity = shader_prog.get_uniform_location("reflectivity");
                // fog unfirom
                location_sky_color = shader_prog.get_uniform_location("sky_color");
                // texture samplers
                location_background_sampler = shader_prog.get_uniform_location("background_sampler");
                location_r_sampler = shader_prog.get_uniform_location("r_sampler");
                location_g_sampler = shader_prog.get_uniform_location("g_sampler");
                location_b_sampler = shader_prog.get_uniform_location("b_sampler");
                location_blend_map_sampler = shader_prog.get_uniform_location("blend_map_sampler");
                // point light attenuation
                location_attenuation = [0i32; NUM_LIGHTS];
                for i in 0..NUM_LIGHTS {
                    location_attenuation[i] = shader_prog.get_uniform_location(&format!("attenuation[{}]", i));
                }
                location_clip_plane = shader_prog.get_uniform_location("clip_plane");
                location_to_shadowmap_space = shader_prog.get_uniform_location("to_shadowmap_space");
                location_shadowmap = shader_prog.get_uniform_location("shadow_map");
                location_shadow_distance = shader_prog.get_uniform_location("shadow_distance");
                location_shadow_map_size = shader_prog.get_uniform_location("shadow_map_size");
        });

        TerrainShader {
            program: shader_program,
            location_transformation_matrix,
            location_projection_matrix,
            location_view_matrix,
            location_light_pos,
            location_light_color,
            location_shine_damper,
            location_reflectivity,
            location_sky_color,
            location_background_sampler,
            location_r_sampler,
            location_g_sampler,
            location_b_sampler,
            location_blend_map_sampler,
            location_attenuation,
            location_clip_plane,
            location_to_shadowmap_space,
            location_shadowmap,
            location_shadow_distance,
            location_shadow_map_size,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_background_sampler, 0);
        ShaderProgram::load_int(self.location_r_sampler, 1);
        ShaderProgram::load_int(self.location_g_sampler, 2);
        ShaderProgram::load_int(self.location_b_sampler, 3);
        ShaderProgram::load_int(self.location_blend_map_sampler, 4);
        ShaderProgram::load_int(self.location_shadowmap, 5);
    }

    pub fn load_sky_color(&mut self, sky_color: &Vector3f) {
        ShaderProgram::load_vector3d(self.location_sky_color, sky_color);
    }

    pub fn load_shine_variables(&mut self, shine_damper: f32, reflectivity: f32) {
        ShaderProgram::load_float(self.location_shine_damper, shine_damper);
        ShaderProgram::load_float(self.location_reflectivity, reflectivity);
    }

    pub fn load_lights(&mut self, lights: &Vec<Light>) {        
        for i in 0..NUM_LIGHTS {
            if i < lights.len() {
                ShaderProgram::load_vector3d(self.location_light_pos[i], &lights[i].position);
                ShaderProgram::load_vector3d(self.location_light_color[i], &lights[i].color);
                ShaderProgram::load_vector3d(self.location_attenuation[i], &lights[i].attenuation);
            } else {
                // no light data means fewer than NUM_LIGHTS affect object
                ShaderProgram::load_vector3d(self.location_light_pos[i], &Vector3f::ZERO);
                ShaderProgram::load_vector3d(self.location_light_color[i], &Vector3f::ZERO);
                ShaderProgram::load_vector3d(self.location_attenuation[i], &Vector3f::POS_X_AXIS);
            }
        } 
    }

    pub fn load_transformation_matrix(&mut self, transform_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_transformation_matrix, transform_matrix);
    }

    pub fn load_projection_matrix(&mut self, projection_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_projection_matrix, projection_matrix);
    }

    pub fn load_view_matrix(&mut self, camera: &Camera) {
        let view_matrix = Matrix4f::create_view_matrix(camera);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }

    pub fn load_clip_plane(&mut self, clip_plane: &Vector4f) {
        ShaderProgram::load_vector4d(self.location_clip_plane, clip_plane);
    }

    pub fn load_to_shadowmap_space(&mut self, to_shadowmap_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_to_shadowmap_space, to_shadowmap_matrix);
    }

    pub fn load_shadow_params(&mut self, shadow_distance: f32, shadow_map_size: usize) {
        ShaderProgram::load_float(self.location_shadow_distance, shadow_distance);
        ShaderProgram::load_float(self.location_shadow_map_size, shadow_map_size as f32);
    }
}
use super::shader_program::ShaderProgram;
use crate::models::{
    RawModel,
};
use crate::entities::{
    Camera,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};

pub struct SkyboxShader {
    program: ShaderProgram,
    location_proj_matrix: i32,
    location_view_matrix: i32,
    location_sky_color: i32,    
    location_cube_map1: i32,    
    location_cube_map2: i32,    
    location_blend_factor: i32,    
}

impl SkyboxShader {
    pub fn new() -> SkyboxShader {
        let (
            mut location_proj_matrix,
            mut location_view_matrix,
            mut location_sky_color,
            mut location_cube_map1,
            mut location_cube_map2,
            mut location_blend_factor,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/skyboxVertexShader.glsl",
            "res/shaders/skyboxFragmentShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            },
            |shader_prog| {
                location_proj_matrix = shader_prog.get_uniform_location("projection_matrix");
                location_view_matrix = shader_prog.get_uniform_location("view_matrix");
                location_sky_color = shader_prog.get_uniform_location("fog_color");
                location_cube_map1 = shader_prog.get_uniform_location("cube_map_sampler1");
                location_cube_map2 = shader_prog.get_uniform_location("cube_map_sampler2");
                location_blend_factor = shader_prog.get_uniform_location("blend_factor");
            }
        );        

        SkyboxShader {
            program,
            location_proj_matrix,
            location_view_matrix,
            location_sky_color,
            location_cube_map1,
            location_cube_map2,
            location_blend_factor,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_cube_map1, 0);
        ShaderProgram::load_int(self.location_cube_map2, 1);
    }

    pub fn load_blend_factor(&mut self, blend_factor: f32) {
        ShaderProgram::load_float(self.location_blend_factor, blend_factor);
    }

    pub fn load_sky_color(&mut self, sky_color: &Vector3f) {
        ShaderProgram::load_vector3d(self.location_sky_color, sky_color);
    }

    pub fn load_projection_matrix(&mut self, projection_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_proj_matrix, projection_matrix);
    }

    pub fn load_view_matrix(&mut self, camera: &Camera, skybox_rotation: f32) {
        let view_matrix = Matrix4f::create_skybox_view_matrix(camera, skybox_rotation);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
}
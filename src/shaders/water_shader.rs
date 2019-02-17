use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
};
use crate::shaders::shader_program::ShaderProgram;

pub struct WaterShader {
    program: ShaderProgram,
    location_proj_mat: i32,
    location_view_mat: i32,
    location_transform_mat: i32,
    location_reflection_unit: i32,
    location_refraction_unit: i32,
    location_dudv_unit: i32,
    location_wave_factor: i32,
}

impl WaterShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
            mut location_view_mat,
            mut location_transform_mat,
            mut location_reflection_unit,
            mut location_refraction_unit,
            mut location_dudv_unit,
            mut location_wave_factor,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/waterVertexShader.glsl",
            "res/shaders/waterFragShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            },
            |shader_prog| {
                location_proj_mat = shader_prog.get_uniform_location("projection_matrix");
                location_view_mat = shader_prog.get_uniform_location("view_matrix");
                location_transform_mat = shader_prog.get_uniform_location("transform_matrix");
                location_reflection_unit = shader_prog.get_uniform_location("reflection_tex");
                location_refraction_unit = shader_prog.get_uniform_location("refraction_tex");
                location_dudv_unit = shader_prog.get_uniform_location("dudv_map");
                location_wave_factor = shader_prog.get_uniform_location("wave_factor");
            },
        );
        WaterShader {
            program,
            location_proj_mat,
            location_view_mat,
            location_transform_mat,
            location_reflection_unit,
            location_refraction_unit,
            location_dudv_unit,
            location_wave_factor,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_projection_matrix(&mut self, proj_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_proj_mat, proj_mat);
    }

    pub fn load_view_matrix(&mut self, view_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_view_mat, view_mat);
    }

    pub fn load_transform_matrix(&mut self, transform_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_transform_mat, transform_mat);
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_reflection_unit, 0);
        ShaderProgram::load_int(self.location_refraction_unit, 1);
        ShaderProgram::load_int(self.location_dudv_unit, 2);
    }

    pub fn load_wave_factor(&mut self, wave_factor: f32) {
        ShaderProgram::load_float(self.location_wave_factor, wave_factor);
    }
}
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
}

impl WaterShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
            mut location_view_mat,
            mut location_transform_mat,
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
            },
        );
        WaterShader {
            program,
            location_proj_mat,
            location_view_mat,
            location_transform_mat,
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
}
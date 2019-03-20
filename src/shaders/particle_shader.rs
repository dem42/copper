use super::shader_program::ShaderProgram;
use crate::math::{
    Matrix4f,
};

pub struct ParticleShader {
    program: ShaderProgram,
    location_proj_mat: i32,
}

impl ParticleShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/particleVertShader.glsl", 
            "res/shaders/particleFragShader.glsl", 
            |shader_program| {

            }, 
            |shader_program| {
                location_proj_mat = shader_program.get_uniform_location("projection_matrix");
            }
        );
        ParticleShader {
            program,
            location_proj_mat,
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
}
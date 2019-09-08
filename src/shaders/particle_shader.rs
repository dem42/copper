use super::shader_program::ShaderProgram;
use crate::math::{
    Matrix4f,
};
use crate::models::{
    RawModel,
    ParticleModel,
    ParticleTexture,
};

pub struct ParticleShader {
    program: ShaderProgram,
    location_proj_mat: i32,
    location_number_of_rows: i32,
}

impl ParticleShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
            mut location_number_of_rows,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/particles/particleVertShader.glsl", 
            None,
            "res/shaders/particles/particleFragShader.glsl", 
            |shader_program| {
                shader_program.bind_attribute(RawModel::POS_ATTRIB, "position");
                shader_program.bind_attribute(ParticleModel::MODELVIEW_COLUMN1, "model_view_matrix");
                shader_program.bind_attribute(ParticleModel::TEX_OFFSET, "tex_offsets");
                shader_program.bind_attribute(ParticleModel::BLEND, "blend_factor");    
            }, 
            |shader_program| {
                location_proj_mat = shader_program.get_uniform_location("projection_matrix");
                location_number_of_rows = shader_program.get_uniform_location("number_of_rows");
            }
        );
        ParticleShader {
            program,
            location_proj_mat,
            location_number_of_rows,
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

    pub fn load_particle_texture_data(&mut self, texture: &ParticleTexture) {
        ShaderProgram::load_float(self.location_number_of_rows, texture.number_of_rows_in_atlas as f32);
    }
}
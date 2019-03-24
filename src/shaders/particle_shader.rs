use super::shader_program::ShaderProgram;
use crate::particles::Particle;
use crate::math::{
    Matrix4f,
    Vector2f,
};
use crate::models::RawModel;

pub struct ParticleShader {
    program: ShaderProgram,
    location_proj_mat: i32,
    location_model_view_mat: i32,
    location_tex_offset1: i32,
    location_tex_offset2: i32,
    location_atlas_data: i32,
}

impl ParticleShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
            mut location_model_view_mat,
            mut location_tex_offset1,
            mut location_tex_offset2,
            mut location_atlas_data,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/particleVertShader.glsl", 
            "res/shaders/particleFragShader.glsl", 
            |shader_program| {
                shader_program.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |shader_program| {
                location_proj_mat = shader_program.get_uniform_location("projection_matrix");
                location_model_view_mat = shader_program.get_uniform_location("model_view_matrix");
                location_tex_offset1 = shader_program.get_uniform_location("tex_offset1");
                location_tex_offset2 = shader_program.get_uniform_location("tex_offset2");
                location_atlas_data = shader_program.get_uniform_location("atlas_data");
            }
        );
        ParticleShader {
            program,
            location_proj_mat,
            location_model_view_mat,
            location_tex_offset1,
            location_tex_offset2,
            location_atlas_data,
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

    pub fn load_model_view_matrix(&mut self, model_view_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_model_view_mat, model_view_mat);
    }

    pub fn load_particle_texture_data(&mut self, particle: &Particle) {
        ShaderProgram::load_vector2d(self.location_tex_offset1, &particle.texture_offset1);
        ShaderProgram::load_vector2d(self.location_tex_offset2, &particle.texture_offset2);
        ShaderProgram::load_vector2d(self.location_atlas_data, &Vector2f::new(particle.texture.number_of_rows_in_atlas as f32, particle.blend));
    }
}
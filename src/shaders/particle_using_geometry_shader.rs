use super::shader_program::ShaderProgram;
use crate::models::RawModel;
use crate::math::{
    Matrix4f,
};

pub struct ParticleUsingGeometryShader {
    program: ShaderProgram,
    location_projection_view_matrix: i32,    
}

impl ParticleUsingGeometryShader {
    pub fn new() -> Self {
        let (
            mut location_projection_view_matrix,
        ) = Default::default();
     
        let shader_program = ShaderProgram::new(
            "res/shaders/particles/simpleParticleVert.glsl", 
            Some("res/shaders/particles/simpleParticleGeo.glsl"),
            "res/shaders/particles/simpleParticleFrag.glsl", 
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
            },
            |shader_prog| {                
                location_projection_view_matrix = shader_prog.get_uniform_location("projectionViewMatrix");
        });

        Self {
            program: shader_program,
            location_projection_view_matrix,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_vp_matrix(&mut self, vp_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_projection_view_matrix, vp_matrix);
    }
}
use crate::math::{
    Matrix4f,
};
use crate::particles::Particle;
use crate::shaders::ParticleShader;

pub struct ParticleRenderer {
    shader: ParticleShader,
}

impl ParticleRenderer {
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        let mut shader = ParticleShader::new();
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.stop();
        ParticleRenderer {
            shader,
        }
    } 

    pub fn render(&mut self, particle: &Particle) {

    }
}
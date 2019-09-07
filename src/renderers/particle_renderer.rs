use std::collections::HashMap;
use crate::entities::Camera;
use crate::models::{
    ParticleTexturedModel,
};
use crate::particles::Particle;

pub trait ParticleRenderer {
    fn render(&mut self, particles: &HashMap<ParticleTexturedModel, Vec<Particle>>, camera: &Camera);
}

pub fn update_vbo(vbo: u32, particle_data: &Vec<f32>) {        
    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data_unitialized::<f32>(gl::ARRAY_BUFFER, particle_data.len(), gl::STREAM_DRAW);
    gl::buffer_sub_data(gl::ARRAY_BUFFER, 0, particle_data);
    gl::bind_buffer(gl::ARRAY_BUFFER, 0);
}
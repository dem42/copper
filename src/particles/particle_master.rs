use crate::constants::GRAVITY;
use crate::display::{
    Display,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::renderers::ParticleRenderer;

pub struct Particle {
    pub position: Vector3f,
    pub velocity: Vector3f,
    pub gravity_effect: f32, // scale that says how much graity affects this particle
    pub rotation: f32,
    pub scale: f32,
    pub lifetime: f32,
    elapsed_time: f32,
}

impl Particle {

    pub fn new(position: Vector3f, velocity: Vector3f, gravity_effect: f32, rotation: f32, scale: f32, lifetime: f32,) -> Self {
        Particle {
            position,
            velocity,
            gravity_effect,
            rotation,
            scale,
            lifetime,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, display: &Display) {
        self.elapsed_time += display.frame_time_sec;        
        self.velocity.y += GRAVITY * display.frame_time_sec;
        let dpos_per_frame = self.velocity.clone() * display.frame_time_sec;
        self.position += &dpos_per_frame;
    }

    pub fn is_alive(&self) -> bool {
        self.elapsed_time < self.lifetime
    }
}

pub struct ParticleMaster {
    particles: Vec<Particle>,    
    particle_renderer: ParticleRenderer,
}

impl ParticleMaster {
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        ParticleMaster {
            particles: Vec::new(),
            particle_renderer: ParticleRenderer::new(projection_matrix),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, display: &Display) {
        for particle in self.particles.iter_mut() {
            particle.update(display);
        }
        self.particles.retain(|particle_ref| particle_ref.is_alive());
    }

    pub fn render(&mut self) {
        for particle in &self.particles {
            self.particle_renderer.render(particle);
        }
    }
}
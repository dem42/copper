use rand::Rng;
use std::f32;
use crate::display::Display;
use crate::math::{
    Vector3f,
};
use crate::models::ParticleModel;
use crate::particles::{
    Particle,
    ParticleMaster,
};

pub trait ParticleSystem {
    fn emit_particles(&self, particle_master: &mut ParticleMaster, spawn_pos: &Vector3f, display: &Display);
}

pub struct SimpleParticleSystem {
    particle_model: ParticleModel,
    particles_per_sec: f32, 
    speed: f32, 
    gravity_effect: f32, 
    life_length: f32,
}

impl SimpleParticleSystem {
    pub fn new(particle_model: ParticleModel, particles_per_sec: f32, speed: f32, gravity_effect: f32, life_length: f32) -> Self {
        SimpleParticleSystem {
            particle_model,
            particles_per_sec, 
            speed, 
            gravity_effect, 
            life_length,
        }
    }

    fn create_particle(&self, spawn_pos: &Vector3f, mut velocity: Vector3f) -> Particle {
        velocity.normalize();
        velocity *= self.speed;
        Particle::new(self.particle_model.clone(), 
            spawn_pos.clone(), 
            velocity, 
            self.gravity_effect, 0.0, 1.0, self.life_length)
    }
}

impl ParticleSystem for SimpleParticleSystem {
    fn emit_particles(&self, particle_master: &mut ParticleMaster, spawn_pos: &Vector3f, display: &Display) {

        let delta = self.particles_per_sec * display.frame_time_sec;
        let count = delta.floor() as usize;
        let percentage_to_spawn = delta % 1.0;
        let mut rng = rand::thread_rng();

        for _ in 0..count {            
            particle_master.add_particle(
                self.create_particle(spawn_pos, Vector3f::new(rng.gen::<f32>() * 2.0 - 1.0, 1.0, rng.gen::<f32>() * 2.0 - 1.0))
            );
        }

        if rng.gen::<f32>() < percentage_to_spawn {
            particle_master.add_particle(
                self.create_particle(spawn_pos, Vector3f::new(rng.gen::<f32>() * 2.0 - 1.0, 1.0, rng.gen::<f32>() * 2.0 - 1.0))
            );
        }
    }
}
use crate::math::{
    Vector3f,
};
use crate::models::ParticleModel;
use crate::particles::{
    Particle,
    ParticleMaster,
};

pub struct ParticleSystem {
    particle_model: ParticleModel,
}

impl ParticleSystem {
    pub fn new(particle_model: ParticleModel) -> Self {
        ParticleSystem {
            particle_model,
        }
    }

    pub fn emit_particles(&self, particle_master: &mut ParticleMaster, spawn_pos: &Vector3f) {
        let new_particle = Particle::new(self.particle_model.clone(), spawn_pos.clone(), Vector3f::new(0.0, 30.0, 0.0), 1.0, 0.0, 1.0, 4.0);
        particle_master.add_particle(new_particle);
    }
}
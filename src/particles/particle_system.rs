use crate::particles::{
    ParticleMaster,
};

pub struct ParticleSystem {

}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {

        }
    }

    pub fn emit_particles(&self, particle_master: &mut ParticleMaster) {
        unimplemented!();
    }
}
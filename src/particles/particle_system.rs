use rand::{
    Rng,
    prelude::ThreadRng,
};
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

pub struct AdvancedParticleSystem {
    particle_model: ParticleModel,
    particles_per_sec: f32, 
    speed: f32, 
    scale: f32,
    gravity_effect: f32, 
    life_length: f32,
    speed_error: f32,
    life_error: f32,
    scale_error: f32,
    randomize_rotation: bool,
    direction: Option<Vector3f>,
    direction_deviation: Option<f32>,
}

impl AdvancedParticleSystem {    

    pub fn new(particle_model: ParticleModel, particles_per_sec: f32, speed: f32, gravity_effect: f32, life_length: f32, scale: f32, speed_error: f32, life_error: f32,
        scale_error: f32, randomize_rotation: bool, direction: Option<(Vector3f, f32)>) -> Self {
        let (direction, direction_deviation) =  if let Some((vector, direction_deviation_angle_deg)) = direction { 
            (Some(vector), Some(direction_deviation_angle_deg / 180.0 * f32::consts::PI))
        } else { 
            (None, None)
        };
        Self {
            particle_model,
            particles_per_sec, 
            speed, 
            scale,
            gravity_effect, 
            life_length,
            speed_error: speed_error * speed,
            life_error: life_error * life_length,
            scale_error: scale_error * scale,
            randomize_rotation,
            direction,
            direction_deviation,
        }
    }

    fn create_particle(&self, rng: &mut ThreadRng, spawn_pos: &Vector3f) -> Particle {        
        let mut velocity = if let Some(ref dir_vec) = self.direction {
            AdvancedParticleSystem::generate_random_direction_within_cone(rng, dir_vec, self.direction_deviation.expect("Must have deviation if has direction"))
        } else {
            AdvancedParticleSystem::generate_random_direction(rng)
        };
        velocity.normalize();
        velocity *= AdvancedParticleSystem::generate_value_using_error(rng, self.speed, self.speed_error);
        let particle_scale = AdvancedParticleSystem::generate_value_using_error(rng, self.scale, self.scale_error);
        let particle_rotation = if self.randomize_rotation { rng.gen::<f32>() * 360.0 } else { 0.0 };
        let particle_life = AdvancedParticleSystem::generate_value_using_error(rng, self.life_length, self.life_error);
        Particle::new(self.particle_model.clone(), spawn_pos.clone(), velocity, self.gravity_effect, particle_rotation, particle_scale, particle_life)
    }

    fn generate_random_direction(rng: &mut ThreadRng) -> Vector3f {
        let z = rng.gen::<f32>() * 2.0 - 1.0;
        let theta = rng.gen::<f32>() * f32::consts::PI * 2.0;
        let radius = (1.0 - z*z).sqrt();
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        Vector3f::new(x, y, z)
    }
 
    fn generate_random_direction_within_cone(rng: &mut ThreadRng, cone_direction: &Vector3f, angle: f32) -> Vector3f {
        let r_angle1 = rng.gen::<f32>() * 2.0 * f32::consts::PI;
        let r_angle2 = rng.gen::<f32>() * angle - angle / 2.0;
        let mut perp1 = cone_direction.perpendicular();
        perp1.normalize();
        let mut perp2 = cone_direction.cross_prod(&perp1);
        perp2.normalize();
        let p1 = perp1 * r_angle1.sin();
        let p2 = perp2 * r_angle1.cos();
        let off = p1 + p2;
        let off = off * r_angle2.sin();
        let cone_vec = cone_direction + off;
        cone_vec
    }

    fn generate_value_using_error(rng: &mut ThreadRng, average: f32, error: f32) -> f32 {
        let r_val = (rng.gen::<f32>() - 0.5) * 2.0;
        average + error * r_val
    }
}

impl ParticleSystem for AdvancedParticleSystem {
    fn emit_particles(&self, particle_master: &mut ParticleMaster, spawn_pos: &Vector3f, display: &Display) {
        let delta = self.particles_per_sec * display.frame_time_sec;
        let count = delta.floor() as usize;
        let percentage_to_spawn = delta % 1.0;
        let mut rng = rand::thread_rng();

        for _ in 0..count {            
            particle_master.add_particle(
                self.create_particle(&mut rng, spawn_pos)
            );
        }

        if rng.gen::<f32>() < percentage_to_spawn {
            particle_master.add_particle(
                self.create_particle(&mut rng, spawn_pos)
            );
        }
    }
}
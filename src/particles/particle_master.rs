use std::cmp::Ordering;
use std::collections::HashMap;

use crate::constants::GRAVITY;
use crate::display::{
    Display,
};
use crate::entities::Camera;
use crate::math::{
    Matrix4f,
    Vector2f,
    Vector3f,
};
use crate::models::{
    ParticleTexturedModel,
};
use crate::renderers::ParticleRenderer;
use crate::utils::insertion_sort;
use super::particle_system::{
    AdvancedParticleSystem,
    ParticleSystem,
};

pub struct Particle {
    pub model: ParticleTexturedModel,
    pub position: Vector3f,
    pub velocity: Vector3f,
    pub gravity_effect: f32, // scale that says how much graity affects this particle
    pub rotation_deg_z: f32,
    pub scale: f32,
    pub lifetime: f32,
    // data for animated atlas -> recalculated every frame
    pub texture_offset1: Vector2f,
    pub texture_offset2: Vector2f,
    pub blend: f32,
    elapsed_time: f32,
    distance_sq_from_camera: f32,
}

impl Particle {

    pub fn new(model: ParticleTexturedModel, position: Vector3f, velocity: Vector3f, gravity_effect: f32, rotation_deg_z: f32, scale: f32, lifetime: f32,) -> Self {
        Particle {
            model,
            position,
            velocity,
            gravity_effect,
            rotation_deg_z,
            scale,
            lifetime,
            elapsed_time: 0.0,
            texture_offset1: Vector2f::zero(),
            texture_offset2: Vector2f::zero(),
            blend: 0.0,
            distance_sq_from_camera: 0.0,
        }
    }

    pub fn update(&mut self, display: &Display, camera: &Camera) {
        self.velocity.y += GRAVITY * display.frame_time_sec * self.gravity_effect;
        let dpos_per_frame = self.velocity.clone() * display.frame_time_sec;
        self.position += &dpos_per_frame;
        self.update_texture_atlas_data();
        self.update_dist(camera);
        self.elapsed_time += display.frame_time_sec;        
    }

    pub fn is_alive(&self) -> bool {
        self.elapsed_time < self.lifetime
    }

    fn update_dist(&mut self, camera: &Camera) {
        self.distance_sq_from_camera = (&camera.position - &self.position).length_squared();
    }

    fn update_texture_atlas_data(&mut self) {
        let life_progression = self.elapsed_time / self.lifetime;
        let atlas_progression = life_progression * (self.model.texture.number_of_rows_in_atlas * self.model.texture.number_of_rows_in_atlas) as f32;
        let index1 = atlas_progression.floor() as usize;
        let index2 = if index1 < self.model.texture.number_of_rows_in_atlas - 1 { index1 + 1 } else { index1 };
        self.blend = atlas_progression % 1.0;
        Particle::calc_tex_offset(&mut self.texture_offset1, index1, self.model.texture.number_of_rows_in_atlas);
        Particle::calc_tex_offset(&mut self.texture_offset2, index2, self.model.texture.number_of_rows_in_atlas);
    }

    fn calc_tex_offset(tex_coord: &mut Vector2f, index: usize, rows_in_atlas: usize) {
        let row = index / rows_in_atlas;
        let column = index % rows_in_atlas;
        tex_coord.x = column as f32 / rows_in_atlas as f32;
        tex_coord.y = row as f32 / rows_in_atlas as f32;
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        self.distance_sq_from_camera == other.distance_sq_from_camera
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        // order by decreasing distance to camera
        other.distance_sq_from_camera.partial_cmp(&self.distance_sq_from_camera)
    }
}

pub struct ParticleMaster {
    particles: HashMap<ParticleTexturedModel, Vec<Particle>>,
    particle_renderer: ParticleRenderer,
}

impl ParticleMaster {
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        ParticleMaster {
            particles: HashMap::new(),
            particle_renderer: ParticleRenderer::new(projection_matrix),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        let entry = self.particles.entry(particle.model.clone()).or_insert(Vec::new());
        entry.push(particle);
    }

    pub fn update(&mut self, display: &Display, camera: &Camera) {
        for (_texture, particles) in self.particles.iter_mut() {
            for particle in particles.iter_mut() {
                particle.update(display, camera);
            }
            insertion_sort(particles);
            particles.retain(|particle_ref| particle_ref.is_alive());
        }
        self.particles.retain(|_key, particles| particles.len() > 0);
    }

    pub fn render(&mut self, camera: &Camera) {
        self.particle_renderer.render(&self.particles, camera);
    }

    pub fn emit_particles(&mut self, particle_systems: &Vec<(AdvancedParticleSystem, Vector3f)>, display: &Display) {
        for (system, pos) in particle_systems {
            system.emit_particles(self, pos, display);
        }
    }
}
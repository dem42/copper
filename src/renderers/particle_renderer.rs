use std::collections::HashMap;
use crate::entities::Camera;
use crate::gl;
use crate::math::{
    Matrix4f,
};
use crate::models::{
    RawModel,
    ParticleModel,
    ParticleTexture,
    ParticleTexturedModel,
};
use crate::particles::Particle;
use crate::shaders::ParticleShader;
use super::master_renderer::RenderGroup;

pub struct ParticleRenderer {
    shader: ParticleShader,
    particle_data: Vec<f32>, 
}

impl ParticleRenderer {
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        let mut shader = ParticleShader::new();
        shader.start();
        shader.load_projection_matrix(projection_matrix);
        shader.stop();
        ParticleRenderer {
            shader,
            particle_data: Vec::with_capacity(ParticleModel::MAX_INSTANCES * ParticleModel::INSTANCED_DATA_LENGTH),
        }
    } 

    pub fn render(&mut self, particles: &HashMap<ParticleTexturedModel, Vec<Particle>>, camera: &Camera) {
        gl::helper::push_debug_group(RenderGroup::PARTICLE_EFFECTS_PASS.id, RenderGroup::PARTICLE_EFFECTS_PASS.name);
        self.prepare();

        let view_mat = Matrix4f::create_view_matrix(camera);

        for (model_texture, particles) in particles {
            gl::bind_vertex_array(model_texture.model.raw_model.vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::enable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN1);
            gl::enable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN2);
            gl::enable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN3);
            gl::enable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN4);
            gl::enable_vertex_attrib_array(ParticleModel::TEX_OFFSET);
            gl::enable_vertex_attrib_array(ParticleModel::BLEND);
            self.bind_texture(&model_texture.texture);

            self.particle_data.clear();

            for particle in particles {
                //ParticleRenderer::create_always_camera_facing_model_view_mat(particle, &view_mat, &mut self.particle_data);
                ParticleRenderer::create_always_camera_facing_model_view_mat(particle, &view_mat, camera, &mut self.particle_data);
                ParticleRenderer::update_texture_data(particle, &mut self.particle_data);
            }
            self.update_vbo(model_texture.model.stream_draw_vbo);

            self.shader.load_particle_texture_data(&model_texture.texture);
            
            gl::draw_arrays_instanced(gl::TRIANGLE_STRIP, 0, model_texture.model.raw_model.vertex_count, particles.len());

            gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::disable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN1);
            gl::disable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN2);
            gl::disable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN3);
            gl::disable_vertex_attrib_array(ParticleModel::MODELVIEW_COLUMN4);
            gl::disable_vertex_attrib_array(ParticleModel::TEX_OFFSET);
            gl::disable_vertex_attrib_array(ParticleModel::BLEND);
            gl::bind_vertex_array(0);
        }

        self.finish_rendering();
        gl::helper::pop_debug_group();
    }

    fn bind_texture(&mut self, texture: &ParticleTexture) {
         gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, texture.tex_id);

        if texture.additive {
            // use additive blending where the colors are always combined
            // this is achieved by always using 1.0 for the destination (already rendered) unlike gl::ONE_MINUS_SRC_ALPHA in alpha blending
            // additive blending is good for effects like magic where we want it to be shinier when there is overlap of particles
            gl::blend_func(gl::SRC_ALPHA, gl::ONE);
        } else {
            gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn prepare(&mut self) {
        self.shader.start();
        // we don't want depth tests to prevent particles from being drawn because they are behind other particles -> draw them on top of each other (overdraw?)        
        // however if we were to disable depth testing completely with disable(gl::DEPTH_TEST) then particles will be drawn on top of everything including terrain
        // we want them not to write into depth buffer (depth_mask(false)) but still get tested
        gl::depth_mask(false);
        gl::enable(gl::BLEND);        
    }
    
    fn finish_rendering(&mut self) {
        gl::depth_mask(true);
        gl::disable(gl::BLEND);
        self.shader.stop();
    }

    fn create_always_camera_facing_model_view_mat(particle: &Particle, view_matrix: &Matrix4f, camera: &Camera, storage_buffer: &mut Vec<f32>) {
        let model_matrix = Matrix4f::create_particle_transform_matrix(&particle.position, particle.rotation_deg_z, particle.scale, camera);
        let model_view_matrix = view_matrix * model_matrix;
        // store column wise
        for col in 0..4 {
            for row in 0..4 {
                storage_buffer.push(model_view_matrix[row][col]);
            }
        }
    }

    fn update_texture_data(particle: &Particle, storage_buffer: &mut Vec<f32>) {
        storage_buffer.push(particle.texture_offset1.x);
        storage_buffer.push(particle.texture_offset1.y);
        storage_buffer.push(particle.texture_offset2.x);
        storage_buffer.push(particle.texture_offset2.y);
        storage_buffer.push(particle.blend);
    }

    fn update_vbo(&mut self, vbo: u32) {
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
        gl::buffer_data_unitialized::<f32>(gl::ARRAY_BUFFER, self.particle_data.len(), gl::STREAM_DRAW);
        gl::buffer_sub_data(gl::ARRAY_BUFFER, 0, &self.particle_data);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
    }
}
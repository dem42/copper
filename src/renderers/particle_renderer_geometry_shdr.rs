use std::collections::HashMap;
use crate::entities::Camera;
use crate::gl;
use crate::math::{
    Matrix4f,
};
use crate::display::{
    Display,
};
use crate::models::{
    RawModel,
    ParticleModel,
    ParticleTexture,
    ParticleTexturedModel,
};
use crate::particles::Particle;
use crate::shaders::particle_using_geometry_shader::ParticleUsingGeometryShader;
use super::master_renderer::RenderGroup;
use super::particle_renderer::{
    update_vbo,
    ParticleRenderer,
};

pub struct ParticleRendererGeometryShader {
    shader: ParticleUsingGeometryShader,
    particle_data: Vec<f32>,
}

impl ParticleRenderer for ParticleRendererGeometryShader {
    fn render(&mut self, particles: &HashMap<ParticleTexturedModel, Vec<Particle>>, _camera: &Camera) {
        gl::helper::push_debug_group(RenderGroup::PARTICLE_EFFECTS_PASS.id, RenderGroup::PARTICLE_EFFECTS_PASS.name);
        self.prepare();

        assert!(particles.len() <= 1);

        self.particle_data.clear();

        let mut model_vao = 0;
        let mut particle_num = 0;
        self.shader.start();
        for (model, particle_vec) in particles {
            particle_num = particle_vec.len();
            model_vao = model.model.raw_model.vao_id;                
            for i in 0..particle_num {
                self.particle_data.push(particle_vec[i].position.x);
                self.particle_data.push(particle_vec[i].position.y);
                self.particle_data.push(particle_vec[i].position.z);                
            }
            update_vbo(model.model.stream_draw_vbo, &self.particle_data);
        }

        gl::bind_vertex_array(model_vao);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        // drawing points
        gl::draw_arrays(gl::POINTS, 0, particle_num);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);

        self.shader.stop();

        gl::helper::pop_debug_group();
    }
}

impl ParticleRendererGeometryShader {
    pub fn new(projection_matrix: &Matrix4f) -> Self {
        let mut shader = ParticleUsingGeometryShader::new();
        shader.start();
        shader.load_vp_matrix(projection_matrix);
        shader.stop();
        ParticleRendererGeometryShader {
            shader,
            // we will use the vbo to write particle positions so vec4
            particle_data: Vec::with_capacity(ParticleModel::MAX_INSTANCES * 3)
        }
    }

    pub fn prepare(&mut self) {
        gl::enable(gl::DEPTH_TEST);
        gl::helper::enable_backface_culling();
        gl::clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        gl::clear_color(0.2, 0.38, 0.31, 0.0);
        gl::enable(gl::VERTEX_PROGRAM_POINT_SIZE);
    }
}
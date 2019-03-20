use crate::entities::Camera;
use crate::gl;
use crate::math::{
    Matrix4f,
};
use crate::models::{
    RawModel,
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

    pub fn render(&mut self, particles: &[Particle], camera: &Camera) {
        self.prepare();

        let view_mat = Matrix4f::create_view_matrix(camera);

        for particle in particles {
            self.render_particle(particle, &view_mat);
        }

        self.finish_rendering();
    }

    fn prepare(&mut self) {
        self.shader.start();
        // we don't want depth tests to prevent particles from being drawn because they are behind other particles -> draw them on top of each other (overdraw?)        
        // however if we were to disable depth testing completely with disable(gl::DEPTH_TEST) then particles will be drawn on top of everything including terrain
        // we want them not to write into depth buffer (depth_mask(false)) but still get tested
        gl::depth_mask(false);
        gl::enable(gl::BLEND);
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    fn render_particle(&mut self, particle: &Particle, view_matrix: &Matrix4f) {
        let model_view_matrix = ParticleRenderer::create_always_camera_facing_model_view_mat(particle, view_matrix);
        self.shader.load_model_view_matrix(&model_view_matrix);

        gl::bind_vertex_array(particle.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::draw_arrays(gl::TRIANGLE_STRIP, 0, particle.model.raw_model.vertex_count);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);
    }

    fn finish_rendering(&mut self) {
        gl::depth_mask(true);
        gl::disable(gl::BLEND);
        self.shader.stop();
    }

    fn create_always_camera_facing_model_view_mat(particle: &Particle, view_matrix: &Matrix4f) -> Matrix4f {
        let model_matrix = Matrix4f::create_particle_transform_matrix(&particle.position, particle.rotation_deg_z, particle.scale, view_matrix);
        let model_view_matrix = view_matrix * model_matrix;
        model_view_matrix
    }
}
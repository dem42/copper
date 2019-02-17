use crate::display::{
    Display,
    Framebuffers,
};
use crate::entities::{
    Camera,
    WaterTile,
};
use crate::gl;
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
};
use crate::shaders::WaterShader;

pub struct WaterRenderer {
    shader: WaterShader,
    wave_factor: f32,
}

impl WaterRenderer {
    const WATER_SPEED: f32 = 0.03;

    pub fn new(projection_mat: &Matrix4f) -> Self {
        let mut shader = WaterShader::new();
        shader.start();
        shader.load_projection_matrix(projection_mat);
        shader.connect_texture_units();
        shader.stop();        
        WaterRenderer {
            shader,
            wave_factor: 0.0,
        }
    }

    pub fn render(&mut self, water_tiles: &Vec<WaterTile>, framebuffers: &Framebuffers, camera: &Camera, display: &Display) {
        self.shader.start();
        let view_matrix = Matrix4f::create_view_matrix(camera);
        self.shader.load_view_matrix(&view_matrix);
        
        self.update_wave_factor(display);
        self.shader.load_wave_factor(self.wave_factor);

        for water_tile in water_tiles {
            let transform_matrix = &water_tile.transform;
            self.shader.load_transform_matrix(transform_matrix);

            gl::bind_vertex_array(water_tile.model.raw_model.vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::active_texture(gl::TEXTURE0);
            gl::bind_texture(gl::TEXTURE_2D, framebuffers.reflection_fbo.color_texture);
            gl::active_texture(gl::TEXTURE1);
            gl::bind_texture(gl::TEXTURE_2D, framebuffers.refraction_fbo.color_texture);
            gl::active_texture(gl::TEXTURE2);
            gl::bind_texture(gl::TEXTURE_2D, water_tile.model.dudv_tex_id);

            gl::draw_arrays(gl::TRIANGLE_STRIP, 0, water_tile.model.raw_model.vertex_count);

            gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::bind_vertex_array(0);
        }
        self.shader.stop();
    }

    fn update_wave_factor(&mut self, display: &Display) {
        self.wave_factor += WaterRenderer::WATER_SPEED * display.frame_time_sec;
        self.wave_factor %= 1.0;
    }
}
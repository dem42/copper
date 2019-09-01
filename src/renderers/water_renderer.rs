use crate::display::{
    Display,
    framebuffers::FboMap,
};
use crate::entities::{
    Camera,
    Light,
    WaterTile,
};
use crate::gl;
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::shaders::WaterShader;
use super::master_renderer::RenderGroup;

pub struct WaterRenderer {
    shader: WaterShader,
    wave_factor: f32,
}

impl WaterRenderer {
    const WATER_SPEED: f32 = 0.03;

    pub fn new(projection_mat: &Matrix4f, sky_color: &Vector3f) -> Self {
        let mut shader = WaterShader::new();
        shader.start();
        shader.load_projection_matrix(projection_mat);
        shader.load_sky_color(sky_color);
        shader.connect_texture_units();
        shader.stop();        
        WaterRenderer {
            shader,
            wave_factor: 0.0,
        }
    }

    pub fn render(&mut self, water_tiles: &Vec<WaterTile>, framebuffers: &FboMap, camera: &Camera, display: &Display, lights: &Vec<Light>) {
        gl::helper::push_debug_group(RenderGroup::DRAW_WATER.id, RenderGroup::DRAW_WATER.name);

        self.shader.start();
        self.shader.load_camera(camera);
        
        self.update_wave_factor(display);
        self.shader.load_wave_factor(self.wave_factor);

        self.shader.load_lights(lights);

        let reflection_fbo = framebuffers.fbos.get(FboMap::REFLECTION_FBO).expect("Must have reflection fbo for water render");
        let refraction_fbo = framebuffers.fbos.get(FboMap::REFRACTION_FBO).expect("Must have refraction fbo for water render");

        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, reflection_fbo.color_texture.expect("ReflectionFbo must have a color attachment"));
        gl::active_texture(gl::TEXTURE1);
        gl::bind_texture(gl::TEXTURE_2D, refraction_fbo.color_texture.expect("RefractionFbo must have a color attachment")); 
        gl::active_texture(gl::TEXTURE4);
        gl::bind_texture(gl::TEXTURE_2D, refraction_fbo.depth_texture.expect("RefractionFbo must have a depth attach"));

        // turn on alpha blending for softer edges (linear blending)
        // this is ok because water rendering happens after terrain/entity rendering so we blend with them
        gl::enable(gl::BLEND);
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        for water_tile in water_tiles {
            let transform_matrix = &water_tile.transform;
            self.shader.load_transform_matrix(transform_matrix);

            gl::bind_vertex_array(water_tile.model.raw_model.vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
            
            gl::active_texture(gl::TEXTURE2);
            gl::bind_texture(gl::TEXTURE_2D, water_tile.model.dudv_tex_id);
            gl::active_texture(gl::TEXTURE3);
            gl::bind_texture(gl::TEXTURE_2D, water_tile.model.normal_map_tex_id);

            gl::draw_arrays(gl::TRIANGLE_STRIP, 0, water_tile.model.raw_model.vertex_count);

            gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::bind_vertex_array(0);
        }

        gl::disable(gl::BLEND);
        self.shader.stop();

        gl::helper::pop_debug_group();
    }

    fn update_wave_factor(&mut self, display: &Display) {
        self.wave_factor += WaterRenderer::WATER_SPEED * display.frame_time_sec;
        self.wave_factor %= 1.0;
    }
}
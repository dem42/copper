use crate::renderers::image_renderer::ImageRenderer;
use crate::display::{
    Display,
    framebuffers::FramebufferObject,
};
use crate::shaders::shader::Shader;

pub struct GenericPostprocess<ShaderType: Shader> {
    shader: ShaderType,
    renderer: ImageRenderer,
}

impl<ShaderType: Shader> GenericPostprocess<ShaderType> {
    pub fn new(mut shader: ShaderType, render_target: Option<FramebufferObject>) -> Self {        
        let renderer = ImageRenderer::new(render_target);
        shader.start();
        shader.init();
        shader.stop();
        Self {
            shader,
            renderer,
        }
    }

    pub fn render_with_one_input(&mut self, source_color_texture: u32, display: &Display) {
        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, source_color_texture);
        self.render(display);
    }

    pub fn render_with_two_inputs(&mut self, source_color_texture_1: u32, source_color_texture_2: u32, display: &Display) {
        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, source_color_texture_1);
        gl::active_texture(gl::TEXTURE1);
        gl::bind_texture(gl::TEXTURE_2D, source_color_texture_2);
        self.render(display);
    }

    pub fn render(&mut self, display: &Display) {
        self.shader.start();
        self.renderer.render_quad(display);
        self.shader.stop();
    }

    pub fn get_output_texture(&self) -> Result<u32, &'static str> {
        self.renderer.get_color_texture()            
    }
}
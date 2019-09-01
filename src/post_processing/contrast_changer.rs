use crate::shaders::ContrastShader;
use crate::renderers::image_renderer::ImageRenderer;
use crate::display::framebuffers::framebuffer_object::FramebufferObject;

pub struct ContrastChanger {
    shader: ContrastShader,
}

impl ContrastChanger {
    pub fn new() -> Self {
        let shader = ContrastShader::new();
        ContrastChanger {
            shader,
        }
    }

    pub fn render(&mut self, fbo: &mut FramebufferObject) {
        gl::active_texture(gl::TEXTURE0);
        let color_texture = fbo.color_texture.expect("To change contrast in post processing you must provide a fbo with color texture");
        gl::bind_texture(gl::TEXTURE_2D, color_texture);
        self.shader.start();
        ImageRenderer::render_quad(None);
        self.shader.stop();
    }
}
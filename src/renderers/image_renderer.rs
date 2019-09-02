use crate::display::{
    Display,
    framebuffers::framebuffer_object::FramebufferObject,
};
use crate::gl;

pub struct ImageRenderer {
    target_fbo: Option<FramebufferObject>,
}

impl ImageRenderer {
    pub fn new(opt_render_target: Option<FramebufferObject>) -> Self {
        ImageRenderer {
            target_fbo: opt_render_target,
        }
    }

    pub fn get_color_texture(&self) -> Result<u32, &'static str> {
        match &self.target_fbo {
            Some(fbo) => {
                match fbo.color_texture {
                    Some(color_tex) => Ok(color_tex),
                    None            => Err("The configured render target fbo does not have a color texture attachment"),
                }
            },
            None => Err("Target fbo doesn't exist. You need to use an image renderer which has a target fbo if you want to get the target texture")
        }
    }

    // renders a quad. takes an optional render target parameter to render to this fbo instead of to the currently active fbo
    pub fn render_quad(&mut self, display: &Display) {
        if let Some(opt_fbo) = self.target_fbo.as_mut() {
            opt_fbo.bind();
        }
        gl::clear(gl::COLOR_BUFFER_BIT);
        gl::draw_arrays(gl::TRIANGLE_STRIP, 0, 4);
        if let Some(_) = self.target_fbo {
            display.restore_default_framebuffer();
        }
    }
}
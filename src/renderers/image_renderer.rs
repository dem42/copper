use crate::display::framebuffers::framebuffer_object::FramebufferObject;
use crate::gl;

pub struct ImageRenderer;

impl ImageRenderer {
    // renders a quad. takes an optional render target parameter to render to this fbo instead of to the currently active fbo
    pub fn render_quad(render_target_fbo: Option<&mut FramebufferObject>) {
        if let Some(opt_fbo) = render_target_fbo {
            opt_fbo.bind();
        }
        gl::clear(gl::COLOR_BUFFER_BIT);
        gl::draw_arrays(gl::TRIANGLE_STRIP, 0, 4);
    }
}
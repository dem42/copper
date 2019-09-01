use super::contrast_changer::ContrastChanger;
use crate::models::{
    RawModel,
    QuadModel,
};
use crate::display::{
    Display,
    framebuffers::FboMap,
};
use crate::gl;
use crate::renderers::master_renderer::RenderGroup;

pub struct PostProcessing {
    quad_model: QuadModel,
    contrast_changer: ContrastChanger,
}

impl PostProcessing {
    pub fn new(quad_model: QuadModel) -> Self {
        PostProcessing {
            quad_model,
            contrast_changer: ContrastChanger::new(),
        }
    }

    // here we setup our chain of post processing steps
    pub fn do_post_processing(&mut self, framebuffers: &mut FboMap, display: &Display) {
        let camera_texture_fbo = framebuffers.fbos.get_mut(FboMap::CAMERA_TEXTURE_FBO).expect("A camera texture must be present for postprocessing");

        gl::helper::push_debug_group(RenderGroup::POST_PROCESSING.id, RenderGroup::POST_PROCESSING.name);
        self.start();
        self.contrast_changer.render(camera_texture_fbo);
        self.end();
        display.restore_default_framebuffer();
        gl::helper::pop_debug_group();
    }

    fn start(&mut self) {
        gl::bind_vertex_array(self.quad_model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable(gl::DEPTH_TEST);
    }

    fn end(&mut self) {
        gl::bind_vertex_array(0);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::enable(gl::DEPTH_TEST);
    }
}
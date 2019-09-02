use super::generic_postprocess::GenericPostprocess;
use crate::display::{
    Display,
    framebuffers::FboMap,
    framebuffers::FramebufferObject,
    framebuffers::FboFlags,
};
use crate::gl;
use crate::models::{
    RawModel,
    QuadModel,
};
use crate::shaders::post_processing::{
    HorizontalBlurShader,
    VerticalBlurShader,
    ContrastShader,
};
use crate::renderers::master_renderer::RenderGroup;

pub struct PostProcessing {
    quad_model: QuadModel,
    contrast_changer: GenericPostprocess<ContrastShader>,
    horizontal_blur: GenericPostprocess<HorizontalBlurShader>,
    vertical_blur: GenericPostprocess<VerticalBlurShader>,
    horizontal_blur_small: GenericPostprocess<HorizontalBlurShader>,
    vertical_blur_small: GenericPostprocess<VerticalBlurShader>,
}

impl PostProcessing {
    pub fn new(quad_model: QuadModel, display: &Display) -> Self {
        let screen_size = display.get_size();
        let width = screen_size.0 as usize;
        let height = screen_size.1 as usize;

        let horizontal_blur = GenericPostprocess::new(HorizontalBlurShader::new(width / 2), 
            Some(FramebufferObject::new(width / 2, height / 2, FboFlags::COLOR_TEX)));
        let vertical_blur = GenericPostprocess::new(VerticalBlurShader::new(height / 2), 
            Some(FramebufferObject::new(width / 2, height / 2, FboFlags::COLOR_TEX)));

        let horizontal_blur_small = GenericPostprocess::new(HorizontalBlurShader::new(width / 8), 
            Some(FramebufferObject::new(width / 8, height / 8, FboFlags::COLOR_TEX)));
        let vertical_blur_small = GenericPostprocess::new(VerticalBlurShader::new(height / 8), 
            Some(FramebufferObject::new(width / 8, height / 8, FboFlags::COLOR_TEX)));
        // this final step upscales this image back to screen size
        let contrast_changer = GenericPostprocess::new(ContrastShader::new(), None);

        PostProcessing {
            quad_model,
            contrast_changer,
            horizontal_blur,
            vertical_blur,
            horizontal_blur_small,
            vertical_blur_small,
        }
    }

    // here we setup our chain of post processing steps
    pub fn do_post_processing(&mut self, framebuffers: &mut FboMap, display: &Display) {
        let camera_texture_fbo = framebuffers.fbos.get_mut(FboMap::CAMERA_TEXTURE_FBO).expect("A camera texture must be present for postprocessing");
        let camera_texture = camera_texture_fbo.color_texture.expect("A camera texture must be present for postprocessing");

        gl::helper::push_debug_group(RenderGroup::POST_PROCESSING.id, RenderGroup::POST_PROCESSING.name);
        self.start();

        /////////////////////////////////////////////        
        // strong gaussian blurring
        /////////////////////////////////////////////
        // self.horizontal_blur.render(camera_texture, display);
        // self.horizontal_blur_small.render(self.horizontal_blur.get_output_texture().unwrap(), display);

        // self.vertical_blur.render(self.horizontal_blur_small.get_output_texture().unwrap(), display);
        // self.vertical_blur_small.render(self.vertical_blur.get_output_texture().unwrap(), display);

        self.contrast_changer.render(camera_texture, display);

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
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
    BrightnessFilterShader,
    CombineShader,
};
use crate::renderers::master_renderer::RenderGroup;

pub struct PostProcessing {
    quad_model: QuadModel,
    contrast_changer: GenericPostprocess<ContrastShader>,
    horizontal_blur: GenericPostprocess<HorizontalBlurShader>,
    vertical_blur: GenericPostprocess<VerticalBlurShader>,
    brightness_filter: GenericPostprocess<BrightnessFilterShader>,
    combine_shader: GenericPostprocess<CombineShader>,
}

impl PostProcessing {
    pub fn new(quad_model: QuadModel, display: &Display) -> Self {
        let screen_size = display.get_size();
        let width = screen_size.0 as usize;
        let height = screen_size.1 as usize;

        let horizontal_blur = GenericPostprocess::new(HorizontalBlurShader::new(width / 5), 
            Some(FramebufferObject::new(width / 5, height / 5, FboFlags::COLOR_TEX)));
        let vertical_blur = GenericPostprocess::new(VerticalBlurShader::new(height / 5), 
            Some(FramebufferObject::new(width / 5, height / 5, FboFlags::COLOR_TEX)));

        // this final step upscales this image back to screen size
        let contrast_changer = GenericPostprocess::new(ContrastShader::new(), None);

        // shaders required for bloom effect
        let combine_shader = GenericPostprocess::new(CombineShader::new(), Some(FramebufferObject::new(width, height, FboFlags::COLOR_TEX)));
        let brightness_filter = GenericPostprocess::new(BrightnessFilterShader::new(), Some(FramebufferObject::new(width / 2, height / 2, FboFlags::COLOR_TEX)));

        PostProcessing {
            quad_model,
            contrast_changer,
            horizontal_blur,
            vertical_blur,
            brightness_filter,
            combine_shader,
        }
    }

    // here we setup our chain of post processing steps
    pub fn do_post_processing(&mut self, framebuffers: &mut FboMap, display: &Display) {
        let camera_texture_fbo = framebuffers.fbos.get_mut(FboMap::CAMERA_TEXTURE_FBO).expect("A camera texture must be present for postprocessing");
        let camera_texture = camera_texture_fbo.color_texture.expect("A camera texture must be present for postprocessing");

        gl::helper::push_debug_group(RenderGroup::POST_PROCESSING.id, RenderGroup::POST_PROCESSING.name);
        self.start();

        self.brightness_filter.render_with_one_input(camera_texture, display);
        self.horizontal_blur.render_with_one_input(self.brightness_filter.get_output_texture().unwrap(), display);
        self.vertical_blur.render_with_one_input(self.horizontal_blur.get_output_texture().unwrap(), display);
        self.combine_shader.render_with_two_inputs(camera_texture, self.vertical_blur.get_output_texture().unwrap(), display);
        self.contrast_changer.render_with_one_input(self.combine_shader.get_output_texture().unwrap(), display);

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
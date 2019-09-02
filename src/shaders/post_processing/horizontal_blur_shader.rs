use super::super::{
    shader::Shader,
    shader_program::ShaderProgram,
};

use crate::models::RawModel;

pub struct HorizontalBlurShader {
    shader_program: ShaderProgram,
    viewport_width: usize,
    location_size: i32,
}


impl HorizontalBlurShader {
    pub fn new(viewport_width: usize) -> Self {
        let mut location_size = Default::default();
        let shader_program = ShaderProgram::new(
            "res/shaders/post_processing/horizBlurVert.glsl", 
            "res/shaders/post_processing/blurFrag.glsl", 
            |shader_prog| { 
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |shader_prog| {
                location_size = shader_prog.get_uniform_location("viewport_width");
            });
        Self {
            shader_program,
            viewport_width,
            location_size,
        }
    }

    fn load_viewport_dimension(&mut self, viewport_width: usize) {
        ShaderProgram::load_float(self.location_size, viewport_width as f32);
    }
}

impl Shader for HorizontalBlurShader {
    fn start(&mut self) {
        self.shader_program.start();
    }

    fn stop(&mut self) {
        self.shader_program.stop();
    }

    fn init(&mut self) {
        self.start();
        self.load_viewport_dimension(self.viewport_width);
        self.stop();
    }
}
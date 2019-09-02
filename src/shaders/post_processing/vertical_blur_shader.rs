use super::super::{
    shader::Shader,
    shader_program::ShaderProgram,
};

use crate::models::RawModel;

pub struct VerticalBlurShader {
    shader_program: ShaderProgram,
    viewport_height: usize,
    location_size: i32,
}


impl VerticalBlurShader {
    pub fn new(viewport_height: usize) -> Self {
        let mut location_size = Default::default();
        let shader_program = ShaderProgram::new(
            "res/shaders/post_processing/vertBlurVert.glsl", 
            "res/shaders/post_processing/blurFrag.glsl", 
            |shader_prog| { 
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |shader_prog| {
                location_size = shader_prog.get_uniform_location("viewport_height");
            });
        Self {
            shader_program,
            viewport_height,
            location_size,
        }
    }

    pub fn load_viewport_dimension(&mut self, viewport_height: usize) {
        ShaderProgram::load_float(self.location_size, viewport_height as f32);
    }
}

impl Shader for VerticalBlurShader {
    fn start(&mut self) {
        self.shader_program.start();
    }

    fn stop(&mut self) {
        self.shader_program.stop();
    }

    fn init(&mut self) {
        self.start();
        self.load_viewport_dimension(self.viewport_height);
        self.stop();
    }
}
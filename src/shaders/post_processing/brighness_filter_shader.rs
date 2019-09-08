use super::super::{
    shader::Shader,
    shader_program::ShaderProgram,
};

use crate::models::RawModel;

pub struct BrightnessFilterShader {
    shader_program: ShaderProgram,
}


impl BrightnessFilterShader {
    pub fn new() -> Self {
        let shader_program = ShaderProgram::new(
            "res/shaders/post_processing/defaultVert.glsl",
            None,
            "res/shaders/post_processing/brightnessFilterFrag.glsl",
            |shader_prog| { 
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |_| {
                
            });
        Self {
            shader_program,
        }
    }
}

impl Shader for BrightnessFilterShader {
    fn start(&mut self) {
        self.shader_program.start();
    }

    fn stop(&mut self) {
        self.shader_program.stop();
    }

    fn init(&mut self) {
    }
}
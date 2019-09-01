use super::shader_program::ShaderProgram;

use crate::models::RawModel;

pub struct ContrastShader {
    shader_program: ShaderProgram,
}


impl ContrastShader {
    pub fn new() -> Self {
        let shader_program = ShaderProgram::new(
            "res/shaders/post_processing/contrastVert.glsl", 
            "res/shaders/post_processing/contrastFrag.glsl", 
            |shader_prog| { 
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |shader_prog| {
                
            });
        ContrastShader {
            shader_program,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }

    pub fn stop(&mut self) {
        self.shader_program.stop();
    }
}
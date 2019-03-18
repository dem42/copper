use super::shader_program::ShaderProgram;
use crate::models::{
    RawModel,
};

pub struct TextShader {
    shader_program: ShaderProgram,
}

impl TextShader {
    pub fn new() -> TextShader {
        let program = ShaderProgram::new(
            "res/shaders/guiTextVertexShader.glsl",
            "res/shaders/guiTextFragShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "tex_coord");
            },
            |_shader_prog| {
            }
        );

        TextShader {
            shader_program: program,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }
    
    pub fn stop(&mut self) {
        self.shader_program.stop();
    }
}



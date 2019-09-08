use super::super::{
    shader::Shader,
    shader_program::ShaderProgram,
};

use crate::models::RawModel;

pub struct CombineShader {
    shader_program: ShaderProgram,
    location_in_texture: i32,
    location_brightness_texture: i32,
}


impl CombineShader {
    pub fn new() -> Self {
        let (mut location_in_texture, mut location_brightness_texture) = Default::default();

        let shader_program = ShaderProgram::new(
            "res/shaders/post_processing/defaultVert.glsl",
            None,
            "res/shaders/post_processing/combineFrag.glsl",
            |shader_prog| { 
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            }, 
            |shader_prog| {
                location_in_texture = shader_prog.get_uniform_location("in_texture");
                location_brightness_texture = shader_prog.get_uniform_location("brightness_tex");
            });
        Self {
            shader_program,
            location_in_texture,
            location_brightness_texture,
        }
    }
}

impl Shader for CombineShader {
    fn start(&mut self) {
        self.shader_program.start();
    }

    fn stop(&mut self) {
        self.shader_program.stop();
    }

    fn init(&mut self) {
        // connect sampler uniforms to texture units
        ShaderProgram::load_int(self.location_in_texture, 0);
        ShaderProgram::load_int(self.location_brightness_texture, 1);
    }
}
use super::shader_program::ShaderProgram;
use crate::math::{
    Vector2f,
    Vector3f,
};
use crate::models::{
    RawModel,
};

pub struct TextShader {
    shader_program: ShaderProgram,
    location_position: i32,
    location_color: i32,
}

impl TextShader {
    pub fn new() -> TextShader {
        let (
            mut location_position,
            mut location_color,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/guiTextVertexShader.glsl",
            "res/shaders/guiTextFragShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "tex_coord");
            },
            |shader_prog| {
                location_position = shader_prog.get_uniform_location("transform");
                location_color = shader_prog.get_uniform_location("color");
            }
        );

        TextShader {
            shader_program: program,
            location_position,
            location_color,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }
    
    pub fn stop(&mut self) {
        self.shader_program.stop();
    }

    pub fn load_position(&mut self, position: &Vector2f) {
        ShaderProgram::load_vector2d(self.location_position, position);
    }

    pub fn load_color(&mut self, color: &Vector3f) {
        ShaderProgram::load_vector3d(self.location_color, color);
    }
}



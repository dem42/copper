use super::shader_program::ShaderProgram;
use crate::guis::{
    TextMaterial,
};
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
    location_width: i32,
    location_edge: i32,
    location_outline_width: i32,
    location_outline_edge: i32,
    location_outline_color: i32,
    location_outline_offset: i32,
}

impl TextShader {
    pub fn new() -> TextShader {
        let (
            mut location_position,
            mut location_color,
            mut location_width,
            mut location_edge,
            mut location_outline_width,
            mut location_outline_edge,
            mut location_outline_color,
            mut location_outline_offset,
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
                location_width = shader_prog.get_uniform_location("width");
                location_edge = shader_prog.get_uniform_location("edge");
                location_outline_width = shader_prog.get_uniform_location("border_width");
                location_outline_edge = shader_prog.get_uniform_location("border_edge");
                location_outline_color = shader_prog.get_uniform_location("border_color");
                location_outline_offset = shader_prog.get_uniform_location("shadow_offset");
            }
        );

        TextShader {
            shader_program: program,
            location_position,
            location_color,
            location_width,
            location_edge,
            location_outline_width,
            location_outline_edge,
            location_outline_color,
            location_outline_offset,
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

    pub fn load_text_material(&mut self, material: &TextMaterial) {
        ShaderProgram::load_vector3d(self.location_color, &material.color);
        ShaderProgram::load_float(self.location_width, material.width);
        ShaderProgram::load_float(self.location_edge, material.edge);
        ShaderProgram::load_float(self.location_outline_width, material.outline_width);
        ShaderProgram::load_float(self.location_outline_edge, material.outline_edge);
        ShaderProgram::load_vector3d(self.location_outline_color, &material.outline_color);
        ShaderProgram::load_vector2d(self.location_outline_offset, &material.offset);
    }
}



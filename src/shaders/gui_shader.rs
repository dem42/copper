use super::shader_program::ShaderProgram;
use crate::models::RawModel;
use crate::math::{
    Matrix4f,
};

pub struct GuiShader {
    program: ShaderProgram,
    location_transformation_matrix: i32,    
}

impl GuiShader {
    pub fn new() -> GuiShader {
        let (
            mut location_transformation_matrix,
        ) = Default::default();
     
        let shader_program = ShaderProgram::new(
            "res/shaders/guiVertexShader.glsl", 
            "res/shaders/guiFragmentShader.glsl", 
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
            },
            |shader_prog| {                
                location_transformation_matrix = shader_prog.get_uniform_location("transform");
        });

        GuiShader {
            program: shader_program,
            location_transformation_matrix,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_transformation_matrix(&mut self, transform_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_transformation_matrix, transform_matrix);
    }
}
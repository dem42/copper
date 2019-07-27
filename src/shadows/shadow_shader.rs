use crate::shaders::shader_program::ShaderProgram;
use crate::math::{
    Matrix4f,
};
use crate::models::RawModel;

pub struct ShadowShader {
    shader_program: ShaderProgram,
    location_mvp_matrix: i32,
}

impl ShadowShader {
    pub fn new() -> Self {
        let (
            mut location_mvp_matrix,
        ) = Default::default();

        let shader_program = ShaderProgram::new(
            "res/shaders/shadows/shadowVertexShader.glsl",
            "res/shaders/shadows/shadowFragmentShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
            },
            |shader_prog| {
                location_mvp_matrix = shader_prog.get_uniform_location("mvp_matrix");
            }
        );
        ShadowShader {
            shader_program,
            location_mvp_matrix,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }

    pub fn stop(&mut self) {
        self.shader_program.stop();
    }

    pub fn load_mvp_matrix(&mut self, mvp: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_mvp_matrix, mvp);
    }
}
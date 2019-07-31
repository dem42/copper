use super::shader_program::ShaderProgram;
use crate::models::RawModel;
use crate::math::Matrix4f;

pub struct DebugShader {
    shader_program: ShaderProgram,
    location_mvp_matrix: i32,
}

impl DebugShader {

    pub fn new() -> Self {
        let mut location_mvp_matrix = 0;
        let shader_program = ShaderProgram::new(
            "res/shaders/test/debugVertShader.glsl",
            "res/shaders/test/debugFragShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
            },
            |shader_prog| {
                location_mvp_matrix = shader_prog.get_uniform_location("mvp_matrix");
            }
        );
        DebugShader {
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
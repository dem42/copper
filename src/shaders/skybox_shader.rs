use super::shader_program::ShaderProgram;
use crate::models::{
    RawModel,
};
use crate::entities::{
    Camera,
};
use crate::math::{
    Matrix4f,
};

pub struct SkyboxShader {
    program: ShaderProgram,
    location_proj_matrix: i32,
    location_view_matrix: i32,
}

impl SkyboxShader {
    pub fn new() -> SkyboxShader {
        let (
            mut location_proj_matrix,
            mut location_view_matrix,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/skyboxVertexShader.glsl",
            "res/shaders/skyboxFragmentShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            },
            |shader_prog| {
                location_proj_matrix = shader_prog.get_uniform_location("projection_matrix");
                location_view_matrix = shader_prog.get_uniform_location("view_matrix");
            }
        );        

        SkyboxShader {
            program,
            location_proj_matrix,
            location_view_matrix,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_projection_matrix(&mut self, projection_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_proj_matrix, projection_matrix);
    }

    pub fn load_view_matrix(&mut self, camera: &Camera) {
        let mut view_matrix = Matrix4f::create_view_matrix(camera);
        // view matrix makes objects move closer to the camera as we move towards them since it includes the negative of the camera translation
        // we dont want the skybox to move as we move around (but we do want it to rotate) so we zero out the translation
        view_matrix[3][0] = 0.0;
        view_matrix[3][1] = 0.0;
        view_matrix[3][2] = 0.0;
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
}
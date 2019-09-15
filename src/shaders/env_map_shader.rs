use super::shader_program::ShaderProgram;
use crate::models::RawModel;
use crate::math::{
    Vector3f,
    Matrix4f,
};

pub struct EnvMapShader {
    shader_program: ShaderProgram,
    location_vp_matrix: i32,
    location_model_matrix: i32,
    location_camera_position: i32,
    location_in_texture: i32,
    location_env_map: i32,
}

impl EnvMapShader {

    pub fn new() -> Self {
        let (
            mut location_vp_matrix,
            mut location_camera_position,
            mut location_model_matrix,
            mut location_in_texture,
            mut location_env_map,
        ) = Default::default();

        let shader_program = ShaderProgram::new(
            "res/shaders/envMapVert.glsl",
            None,
            "res/shaders/envMapFrag.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "tex_coord");
                shader_prog.bind_attribute(RawModel::NORMAL_ATTRIB, "normal");
            },
            |shader_prog| {
                location_vp_matrix = shader_prog.get_uniform_location("vp_matrix");
                location_model_matrix = shader_prog.get_uniform_location("model_matrix");
                location_camera_position = shader_prog.get_uniform_location("camera_position");
                location_in_texture = shader_prog.get_uniform_location("in_texture");
                location_env_map = shader_prog.get_uniform_location("env_map");
            }
        );
        Self {
            shader_program,
            location_vp_matrix,
            location_model_matrix,
            location_camera_position,
            location_in_texture,
            location_env_map,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }

    pub fn stop(&mut self) {
        self.shader_program.stop();
    }

    pub fn load_vp_matrix(&mut self, vp: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_vp_matrix, vp);
    }

    pub fn load_model_matrix(&mut self, model: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_model_matrix, model);
    }

    pub fn load_camera_position(&mut self, camera_pos: &Vector3f) {
        ShaderProgram::load_vector3d(self.location_camera_position, camera_pos);
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_in_texture, 0);
        ShaderProgram::load_int(self.location_env_map, 1);
    }
}
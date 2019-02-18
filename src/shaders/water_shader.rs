use crate::entities::{
    Camera,
    Light,
};
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::shaders::shader_program::ShaderProgram;

const LIGHT_NUM: usize = 4;

pub struct WaterShader {
    program: ShaderProgram,
    location_proj_mat: i32,
    location_view_mat: i32,
    location_transform_mat: i32,
    location_reflection_unit: i32,
    location_refraction_unit: i32,
    location_dudv_unit: i32,
    location_wave_factor: i32,
    location_camera: i32,
    location_normal_map_unit: i32,    
    location_light_color: [i32; LIGHT_NUM],
    location_light_pos: [i32; LIGHT_NUM],
    location_attenuation: [i32; LIGHT_NUM],
}

impl WaterShader {
    pub fn new() -> Self {
        let (
            mut location_proj_mat,
            mut location_view_mat,
            mut location_transform_mat,
            mut location_reflection_unit,
            mut location_refraction_unit,
            mut location_dudv_unit,
            mut location_wave_factor,
            mut location_camera,
            mut location_normal_map_unit,
        ) = Default::default();
        let (
            mut location_light_color,
            mut location_light_pos,
            mut location_attenuation,
        ) = Default::default();

        let program = ShaderProgram::new(
            "res/shaders/waterVertexShader.glsl",
            "res/shaders/waterFragShader.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "position");
            },
            |shader_prog| {
                location_proj_mat = shader_prog.get_uniform_location("projection_matrix");
                location_view_mat = shader_prog.get_uniform_location("view_matrix");
                location_transform_mat = shader_prog.get_uniform_location("transform_matrix");
                location_reflection_unit = shader_prog.get_uniform_location("reflection_tex");
                location_refraction_unit = shader_prog.get_uniform_location("refraction_tex");
                location_dudv_unit = shader_prog.get_uniform_location("dudv_map");
                location_wave_factor = shader_prog.get_uniform_location("wave_factor");
                location_camera = shader_prog.get_uniform_location("camera_world_pos");
                location_normal_map_unit = shader_prog.get_uniform_location("normal_map");

                location_light_color = [0i32; LIGHT_NUM];
                location_light_pos = [0i32; LIGHT_NUM];
                location_attenuation = [0i32; LIGHT_NUM];
                for i in 0..LIGHT_NUM {
                    location_light_color[i] = shader_prog.get_uniform_location(&format!("light_color[{}]", i));
                    location_light_pos[i] = shader_prog.get_uniform_location(&format!("light_pos[{}]", i));
                    location_attenuation[i] = shader_prog.get_uniform_location(&format!("attenuation[{}]", i));                
                }
            },
        );
        WaterShader {
            program,
            location_proj_mat,
            location_view_mat,
            location_transform_mat,
            location_reflection_unit,
            location_refraction_unit,
            location_dudv_unit,
            location_wave_factor,
            location_camera,
            location_normal_map_unit,
            location_light_color,
            location_light_pos,
            location_attenuation,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_projection_matrix(&mut self, proj_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_proj_mat, proj_mat);
    }

    pub fn load_camera(&mut self, camera: &Camera) {
        let view_matrix = Matrix4f::create_view_matrix(camera);
        ShaderProgram::load_matrix(self.location_view_mat, &view_matrix);
        ShaderProgram::load_vector3d(self.location_camera, &camera.position);
    }

    pub fn load_transform_matrix(&mut self, transform_mat: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_transform_mat, transform_mat);
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_reflection_unit, 0);
        ShaderProgram::load_int(self.location_refraction_unit, 1);
        ShaderProgram::load_int(self.location_dudv_unit, 2);
        ShaderProgram::load_int(self.location_normal_map_unit, 3);
    }

    pub fn load_wave_factor(&mut self, wave_factor: f32) {
        ShaderProgram::load_float(self.location_wave_factor, wave_factor);
    }

    pub fn load_lights(&mut self, lights: &Vec<Light>) {
        for li in 0..LIGHT_NUM {
            if li < lights.len() {
                ShaderProgram::load_vector3d(self.location_light_color[li], &lights[li].color);
                ShaderProgram::load_vector3d(self.location_light_pos[li], &lights[li].position);
                ShaderProgram::load_vector3d(self.location_attenuation[li], &lights[li].attenuation);
            } else {
                ShaderProgram::load_vector3d(self.location_light_color[li], &Vector3f::ZERO);
                ShaderProgram::load_vector3d(self.location_light_pos[li], &Vector3f::ZERO);
                ShaderProgram::load_vector3d(self.location_attenuation[li], &Vector3f::POS_X_AXIS);
            }
        }
    }
}
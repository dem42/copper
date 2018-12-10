use super::shader_program::ShaderProgram;
use super::super::entities::{
    Camera,
    Light,
};
use super::super::loader::RawModel;
use super::super::math::Matrix4f;

pub struct StaticShader {
    program: ShaderProgram,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
    location_light_pos: i32,
    location_light_color: i32,
    location_shine_damper: i32,
    location_reflectivity: i32,
    location_uses_fake_lighting: i32,
}

impl StaticShader {
    pub fn new() -> StaticShader {
        let (
            mut location_transformation_matrix, 
            mut location_projection_matrix,
            mut location_view_matrix,
            mut location_light_pos,
            mut location_light_color,
            mut location_shine_damper,
            mut location_reflectivity,
            mut location_uses_fake_lighting,
        ) = Default::default();
        
        let shader_program = ShaderProgram::new(
            String::from("res/shaders/vertexShader.glsl"), 
            String::from("res/shaders/fragmentShader.glsl"), 
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "pos");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "tex_coord");
                shader_prog.bind_attribute(RawModel::NORMAL_ATTRIB, "normal");
            },
            |shader_prog| {                
                location_transformation_matrix = shader_prog.get_uniform_location("transform");
                location_projection_matrix = shader_prog.get_uniform_location("projection_matrix");
                location_view_matrix = shader_prog.get_uniform_location("view_matrix");
                // diffuse lighting
                location_light_pos = shader_prog.get_uniform_location("light_pos");
                location_light_color = shader_prog.get_uniform_location("light_color");
                // specular lighting
                location_shine_damper = shader_prog.get_uniform_location("shine_damper");
                location_reflectivity = shader_prog.get_uniform_location("reflectivity");
                // bad grass model hack
                location_uses_fake_lighting = shader_prog.get_uniform_location("uses_fake_lighting");
        });

        StaticShader {
            program: shader_program,
            location_transformation_matrix,
            location_projection_matrix,
            location_view_matrix,
            location_light_pos,
            location_light_color,
            location_shine_damper,
            location_reflectivity,
            location_uses_fake_lighting,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
    }

    pub fn load_uses_fake_lighting(&mut self, uses_fake: bool) {
        ShaderProgram::load_bool(self.location_uses_fake_lighting, uses_fake);
    }

    pub fn load_shine_variables(&mut self, shine_damper: f32, reflectivity: f32) {
        ShaderProgram::load_float(self.location_shine_damper, shine_damper);
        ShaderProgram::load_float(self.location_reflectivity, reflectivity);
    }

    pub fn load_light(&mut self, light: &Light) {
        ShaderProgram::load_vector(self.location_light_pos, &light.position);
        ShaderProgram::load_vector(self.location_light_color, &light.color);
    }

    pub fn load_transformation_matrix(&mut self, transform_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_transformation_matrix, transform_matrix);
    }

    pub fn load_projection_matrix(&mut self, projection_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_projection_matrix, projection_matrix);
    }

    pub fn load_view_matrix(&mut self, camera: &Camera) {
        let view_matrix = Matrix4f::create_view_matrix(camera);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
}
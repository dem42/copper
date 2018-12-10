use super::shader_program::ShaderProgram;
use super::super::entities::{
    Camera,
    Light,
};
use super::super::loader::RawModel;
use super::super::math::Matrix4f;

pub struct TerrainShader {
    program: ShaderProgram,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
    location_light_pos: i32,
    location_light_color: i32,
    location_shine_damper: i32,
    location_reflectivity: i32,
}

impl TerrainShader {
    pub fn new() -> TerrainShader {
        let (
            mut location_transformation_matrix, 
            mut location_projection_matrix,
            mut location_view_matrix,
            mut location_light_pos,
            mut location_light_color,
            mut location_shine_damper,
            mut location_reflectivity,
        ) = Default::default();
        
        let shader_program = ShaderProgram::new(
            String::from("res/shaders/terrainVertexShader.glsl"), 
            String::from("res/shaders/terrainFragShader.glsl"), 
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
        });

        TerrainShader {
            program: shader_program,
            location_transformation_matrix,
            location_projection_matrix,
            location_view_matrix,
            location_light_pos,
            location_light_color,
            location_shine_damper,
            location_reflectivity,
        }
    }

    pub fn start(&mut self) {
        self.program.start();
    }

    pub fn stop(&mut self) {
        self.program.stop();
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
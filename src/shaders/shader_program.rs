use std::fs::File;
// items from traits can only be used if trait is in scope
// we need io traits which are exported in bulk in prelude
use std::io::{
    prelude::*,
    Error,
    ErrorKind,
    BufReader,
}; 
use super::super::gl;
use super::super::loader::RawModel;
use super::super::math::{Vector3f, Matrix4f};
use super::super::camera::Camera;

pub struct StaticShader {
    program: ShaderProgram,
    location_transformation_matrix: i32,
    location_projection_matrix: i32,
    location_view_matrix: i32,
}

impl StaticShader {
    pub fn new(model: &RawModel) -> StaticShader {

        let mut location_transformation_matrix = 0;
        let mut location_projection_matrix = 0;
        let mut location_view_matrix = 0;
        let shader_program = ShaderProgram::new(
            String::from("src/shaders/vertexShader.glsl"), 
            String::from("src/shaders/fragmentShader.glsl"), 
            |shader_prog| {
                shader_prog.bind_attribute(model.pos_attrib, "pos");
                shader_prog.bind_attribute(model.tex_coord_attrib, "tex_coord");
            },
            |shader_prog| {
                location_transformation_matrix = shader_prog.get_uniform_location("transform");
                location_projection_matrix = shader_prog.get_uniform_location("projection_matrix");
                location_view_matrix = shader_prog.get_uniform_location("view_matrix");
        });

        StaticShader {
            program: shader_program,
            location_transformation_matrix,
            location_projection_matrix,
            location_view_matrix,
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

    pub fn load_projection_matrix(&mut self, projection_matrix: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_projection_matrix, projection_matrix);
    }

    pub fn load_view_matrix(&mut self, camera: &Camera) {
        let view_matrix = Matrix4f::create_view_matrix(camera);
        ShaderProgram::load_matrix(self.location_view_matrix, &view_matrix);
    }
}

struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
}

impl ShaderProgram {

    pub fn new<F1, F2>(vertex_file: String, fragment_file: String, attrib_binder_fn: F1, uniform_loader: F2) -> ShaderProgram 
        where F1: FnOnce(&ShaderProgram) -> (), 
              F2: FnOnce(&ShaderProgram) -> () {
        let vertex_shader_id = ShaderProgram::load_shader(vertex_file, gl::VERTEX_SHADER)
            .expect("Failed to create vertex shader");
        let fragment_shader_id = ShaderProgram::load_shader(fragment_file, gl::FRAGMENT_SHADER)
            .expect("Failed to create fragment shader");
        let program_id = gl::create_program();
        gl::attach_shader(program_id, vertex_shader_id);
        gl::attach_shader(program_id, fragment_shader_id);
        
        let shader_prog = ShaderProgram {
            program_id,
            vertex_shader_id,
            fragment_shader_id,
        };
        attrib_binder_fn(&shader_prog);
        gl::link_program(program_id);
        if gl::get_program(program_id, gl::LINK_STATUS) == gl::FALSE as i32 {
            let link_log = gl::get_program_info_log(program_id).expect("Failed to get program log");
            println!("Link log: {}", link_log);
            panic!("Program linking failed");
        }
        gl::validate_program(program_id);
        if gl::get_program(program_id, gl::VALIDATE_STATUS) == gl::FALSE as i32 {
            let validate_log = gl::get_program_info_log(program_id).expect("Failed to get program log");
            println!("Validate log: {}", validate_log);
            panic!("Program linking failed");
        }
        uniform_loader(&shader_prog);
        shader_prog
    }

    fn start(&self) {
        gl::use_program(self.program_id);
    }

    fn stop(&self) {
        gl::use_program(0);
    }

    fn load_shader(filename: String, type_: u32) -> std::io::Result<u32> {
        let shader_file = File::open(filename)?;
        let mut buf_reader = BufReader::new(shader_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let shader_id = gl::create_shader(type_);
        gl::shader_source(shader_id, &contents)?;
        gl::compile_shader(shader_id);
        if gl::get_shader(shader_id, gl::COMPILE_STATUS) == gl::FALSE as i32 {
            let compile_log = gl::get_shader_info_log(shader_id)?;
            println!("Could not compile shader. Log: {}", compile_log);
            Err(Error::new(ErrorKind::Other, "Failed to compile shader"))
        }
        else {
            Ok(shader_id)
        }
    }

    fn bind_attribute(&self, attribute: u32, variable_name: &str) {
        gl::bind_attrib_location(self.program_id, attribute, variable_name).expect("Variable name invalid");
    }
    
    fn get_uniform_location(&self, uniform_name: &str) -> i32 {
        gl::get_uniform_location(self.program_id, uniform_name).expect("Couldn't get uniform location")
    }

    fn load_float(location_id: i32, value: f32) {
        gl::uniform1f(location_id, value);
    }

    fn load_bool(location_id: i32, value: bool) {
        gl::uniform1f(location_id, if value { 1.0 } else { 0.0 });
    }

    fn load_vector(location_id: i32, value: &Vector3f) {
        gl::uniform3f(location_id, value.x, value.y, value.z);
    }

    fn load_matrix(location_id: i32, value: &Matrix4f) {
        gl::uniform_matrix4f(location_id, value.data());
    } 
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.stop();
        gl::detach_shader(self.program_id, self.vertex_shader_id);
        gl::detach_shader(self.program_id, self.fragment_shader_id);
        gl::delete_shader(self.vertex_shader_id);
        gl::delete_shader(self.fragment_shader_id);
        gl::delete_program(self.program_id);
    }
}

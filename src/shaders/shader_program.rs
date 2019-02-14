use std::fs::File;
// items from traits can only be used if trait is in scope
// we need io traits which are exported in bulk in prelude
use std::io::{
    prelude::*,
    BufReader,
    Error,
    ErrorKind,
}; 
use super::super::gl;
use super::super::math::{
    Matrix4f,
    Vector2f, 
    Vector3f,
    Vector4f,
};

pub struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
}

impl ShaderProgram {

    pub fn new<F1, F2>(vertex_file: &str, fragment_file: &str, attrib_binder_fn: F1, uniform_loader: F2) -> ShaderProgram 
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

    pub fn start(&self) {
        gl::use_program(self.program_id);
    }

    pub fn stop(&self) {
        gl::use_program(0);
    }

    pub fn load_shader(filename: &str, type_: u32) -> std::io::Result<u32> {
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

    pub fn bind_attribute(&self, attribute: u32, variable_name: &str) {
        gl::bind_attrib_location(self.program_id, attribute, variable_name).expect("Variable name invalid");
    }
    
    pub fn get_uniform_location(&self, uniform_name: &str) -> i32 {
        gl::get_uniform_location(self.program_id, uniform_name).expect("Couldn't get uniform location")
    }

    ////////////////////////////////////////////////////////////////////
    /// The following are associated functions not members
    /// they don't need the program_id but they do need the program 
    /// to be started (self.start())
    ////////////////////////////////////////////////////////////////////
    pub fn load_float(location_id: i32, value: f32) {
        gl::uniform1f(location_id, value);
    }

    pub fn load_int(location_id: i32, value: i32) {
        gl::uniform1i(location_id, value);
    }

    pub fn load_bool(location_id: i32, value: bool) {
        gl::uniform1f(location_id, if value { 1.0 } else { 0.0 });
    }

    pub fn load_vector4d(location_id: i32, value: &Vector4f) {
        gl::uniform4f(location_id, value.x, value.y, value.z, value.w);
    }

    pub fn load_vector3d(location_id: i32, value: &Vector3f) {
        gl::uniform3f(location_id, value.x, value.y, value.z);
    }

    pub fn load_vector2d(location_id: i32, value: &Vector2f) {
        gl::uniform2f(location_id, value.x, value.y);
    }

    pub fn load_matrix(location_id: i32, value: &Matrix4f) {
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

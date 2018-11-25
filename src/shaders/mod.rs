use std::fs::File;
// items from traits can only be used if trait is in scope
// we need io traits which are exported in bulk in prelude
use std::io::{
    prelude::*,
    Error,
    ErrorKind,
    BufReader,
}; 
use super::gl;
use super::loader::RawModel;

pub fn create_static_shader_for_model(model: &RawModel) -> ShaderProgram {
    
    let shader_program = ShaderProgram::new(
        String::from("src/shaders/vertexShader.glsl"), 
        String::from("src/shaders/fragmentShader.glsl"), 
        |shader_prog| {
            shader_prog.bind_attribute(model.attribute_id, String::from("position"));
        });

    shader_program
}

pub struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
}

impl ShaderProgram {

    pub fn new<T>(vertex_file: String, fragment_file: String, attrib_binder_fn: T) -> ShaderProgram 
        where T: FnOnce(&ShaderProgram) -> () {
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
        shader_prog
    }

    pub fn start(&self) {
        gl::use_program(self.program_id);
    }

    pub fn stop(&self) {
        gl::use_program(0);
    }

    fn load_shader(filename: String, type_: u32) -> std::io::Result<u32> {
        let shader_file = File::open(filename)?;
        let mut buf_reader = BufReader::new(shader_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let shader_id = gl::create_shader(type_);
        gl::shader_source(shader_id, contents)?;
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

    fn bind_attribute(&self, attribute: u32, variable_name: String) {
        gl::bind_attrib_location(self.program_id, attribute, variable_name).expect("Variable name invalid");
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

include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::io::{
    Error,
    ErrorKind,
};

///////////
// gl 1.1
///////////
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        ClearColor(r, g, b, a);
    }
}

pub fn clear(mask: u32) {
    unsafe {
        Clear(mask);
    }
}

pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        Viewport(x, y, width, height);
    }
}

pub fn draw_arrays(draw_type: types::GLenum, first_idx: usize, num_to_draw: usize) {
    unsafe {
        DrawArrays(draw_type, first_idx as i32, num_to_draw as i32);
    }
}

pub fn draw_elements(draw_mode: types::GLenum, index_cnt: usize, draw_type: types::GLenum) {
    unsafe {
        let offset = ptr::null() as *const _; // offset to start of data in buffer
        DrawElements(draw_mode, index_cnt as i32, draw_type, offset);
    }
}

pub fn enable(capability: types::GLenum) {
    unsafe {
        Enable(capability);
    }
}

pub fn disable(capability: types::GLenum) {
    unsafe {
        Disable(capability);
    }
}

pub fn cull_face(type_: types::GLenum) {
    unsafe {
        CullFace(type_);
    }
}

///////////
// gl 1.3
///////////

pub fn active_texture(bank_type: types::GLenum) {
    unsafe {
        ActiveTexture(bank_type);
    }
}


///////////
// gl 1.5
///////////
pub fn gen_buffer() -> u32 {
    unsafe {
        let mut buffers = [0u32; 1];
        let buffers_ptr = buffers.as_mut_ptr();
        GenBuffers(1, buffers_ptr);
        buffers[0]
    }
}

pub fn bind_buffer(kind: types::GLenum, buffer_id: u32) {
    unsafe {
        BindBuffer(kind, buffer_id);
    }
}

///
/// Set data into the currently bound vbo
///
/// usage this tells us whether the data will be static or if we will change it later
///
pub fn buffer_data<T>(target: types::GLenum, data: &[T], usage: types::GLenum) {
    unsafe {
        let size_in_bytes = (data.len() * mem::size_of::<T>()) as isize;
        let data_ptr = data.as_ptr();
        BufferData(target, size_in_bytes, data_ptr as *const _, usage);
    }
}

pub fn delete_buffers(buffer_ids: &[u32]) {
    unsafe {
        DeleteBuffers(buffer_ids.len() as i32, buffer_ids.as_ptr());
    }
}

///////////
// gl 2.0
///////////
pub fn get_uniform_location(program_id: u32, unifrom_name: &str) -> Result<i32, Error> {
    let unifrom_name_nul_term = CString::new(unifrom_name)?;
    let uniform_loc = unsafe {
         GetUniformLocation(program_id, unifrom_name_nul_term.as_ptr())
    };
    if uniform_loc == -1 {
        Err(Error::new(ErrorKind::Other, format!("Couldn't find uniform: {}", unifrom_name)))
    }
    else {
        Ok(uniform_loc)
    }
}

pub fn uniform1f(location_id: i32, value: f32) {
    unsafe {
        Uniform1f(location_id, value);
    }
}

pub fn uniform1i(location_id: i32, value: i32) {
    unsafe {
        Uniform1i(location_id, value);
    }
}

pub fn uniform3f(location_id: i32, x: f32, y: f32, z: f32) {
    unsafe {
        Uniform3f(location_id, x, y, z);
    }
}

pub fn uniform_matrix4f(location_id: i32, matrix: &[[f32; 4]; 4]) {
    unsafe {        
        // hope this cast is ok .. in memory the 4x4 array should be just a block of 16 floats
        // in row major order since rust uses row major 
        let transpose = TRUE; // true implies row major order (is rust always gonna be row major?)
        UniformMatrix4fv(location_id, 1, transpose, matrix.as_ptr() as *const f32);
    }
}
 
pub fn vertex_attrib_pointer(
    attribute_id: u32,
    components_per_attribute: u32,
    data_type: types::GLenum,
) {
    unsafe {
        let should_normalize = false as u8;
        let stride = 0; // dist between vertices
        let offset = ptr::null() as *const _; // offset to start of data in buffer
        VertexAttribPointer(
            attribute_id,
            components_per_attribute as i32,
            data_type,
            should_normalize,
            stride,
            offset,
        );
    }
}

pub fn enable_vertex_attrib_array(attribute_id: u32) {
    unsafe {
        EnableVertexAttribArray(attribute_id);
    }
}

pub fn disable_vertex_attrib_array(attribute_id: u32) {
    unsafe {
        DisableVertexAttribArray(attribute_id);
    }
}

pub fn create_shader(type_: types::GLenum) -> u32 {
    unsafe {
        CreateShader(type_)
    }
}

pub fn shader_source(shader_id: u32, file: &str) -> Result<(), Error> {
    let file_contents_cstr = match CString::new(file) {
        Ok(contents) => contents,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Shader source code contains nul byte")),
    };
    let files: Vec<*const c_char> = vec![file_contents_cstr.into_raw()];
    let lengths: *const i32 = ptr::null(); // null means that all files are null terminated strings
    let file_num = 1;
    unsafe {
        let files_ptr = files.as_ptr() as *const *const c_char;
        ShaderSource(shader_id, file_num, files_ptr, lengths);    
    }
    Ok(())    
}

pub fn compile_shader(shader_id: u32) {
    unsafe {
        CompileShader(shader_id);
    }
}

pub fn get_shader(shader_id: u32, param_name: types::GLenum) -> i32 {
    unsafe {
        let mut param_result: i32 = 0;
        GetShaderiv(shader_id, param_name, &mut param_result as *mut i32);
        param_result
    }
}

pub fn get_shader_info_log(shader_id: u32) -> Result<String, Error> {
    let log_len = get_shader(shader_id, INFO_LOG_LENGTH);
    let mut log_bytes = vec![0u8; log_len as usize];
    unsafe {
        GetShaderInfoLog(shader_id, log_len, ptr::null_mut(), log_bytes.as_mut_ptr() as *mut c_char);
    }
    let res = CString::new(&log_bytes[..(log_len-1) as usize])?;
    match res.into_string() {
        Err(_) => Err(Error::new(ErrorKind::Other, "Shader info log not valid utf8")),
        Ok(st) => Ok(st),
    }
}

pub fn create_program() -> u32 {
    unsafe {
        CreateProgram()
    }
}

pub fn attach_shader(program_id: u32, shader_id: u32) {
    unsafe {
        AttachShader(program_id, shader_id);
    }
}

pub fn link_program(program_id: u32) {
    unsafe {
        LinkProgram(program_id);
    }
}

pub fn validate_program(program_id: u32) {
    unsafe {
        ValidateProgram(program_id);
    }
}

pub fn use_program(program_id: u32) {
    unsafe {
        UseProgram(program_id);
    }
}

pub fn detach_shader(program_id: u32, shader_id: u32) {
    unsafe {
        DetachShader(program_id, shader_id);
    }
}

pub fn delete_shader(shader_id: u32) {
    unsafe {
        DeleteShader(shader_id);
    }
}

pub fn delete_program(program_id: u32) {
    unsafe {
        DeleteProgram(program_id);
    }
}

pub fn get_program(program_id: u32, param_name: types::GLenum) -> i32 {
    unsafe {
        let mut param_result: i32 = 0;
        GetProgramiv(program_id, param_name, &mut param_result as *mut i32);
        param_result
    }
}

pub fn get_program_info_log(program_id: u32) -> Result<String, Error> {
    let log_len = get_program(program_id, INFO_LOG_LENGTH);
    let mut log_bytes = vec![0u8; log_len as usize];
    unsafe {
        GetProgramInfoLog(program_id, log_len, ptr::null_mut(), log_bytes.as_mut_ptr() as *mut c_char);
    }
    // todo: why does this fail if we do ::new ?
    let res = CString::new(&log_bytes[..(log_len-1) as usize])?;
    match res.into_string() {
        Err(_) => Err(Error::new(ErrorKind::Other, "Program info log not valid utf8")),
        Ok(st) => Ok(st),
    }
}

pub fn bind_attrib_location(program_id: u32, attribute_id: u32, variable_name: &str) -> Result<(), Error> {
    let variable_name_nul_term = CString::new(variable_name)?;
    unsafe {        
        BindAttribLocation(program_id, attribute_id, variable_name_nul_term.as_ptr());
    }
    Ok(())
}

pub fn gen_texture() -> u32 {
    unsafe {
        let mut textures = [0u32; 1];
        let textures_ptr = textures.as_mut_ptr();
        GenTextures(1, textures_ptr);
        textures[0]
    }
}

pub fn bind_texture(texture_id: u32, type_: types::GLenum) {
    unsafe {
        BindTexture(type_, texture_id);
    }
}

pub fn tex_image_2d<T>(type_: types::GLenum, level_of_detail: i32, format: types::GLenum, width: usize, height: usize, pixel_format: types::GLenum, data: &[T]) {
    unsafe {
        
        //TexImage2D(type_, level_of_detail, format as i32, width as i32, height as i32, 0, format, pixel_format, data.as_ptr() as *const _);
        // let pixels = vec![
        //     0.0, 0.0, 0.0,
        //     1.0, 1.0, 1.0,
        //     1.0, 1.0, 1.0,
        //     0.0, 0.0, 0.0,
        // ];
        // TexImage2D(TEXTURE_2D, 0, RGB as i32, 2, 2, 0, RGB, FLOAT, pixels.as_ptr() as *const _);
        TexImage2D(type_, level_of_detail, format as i32, width as i32, height as i32, 0, format, pixel_format, data.as_ptr() as *const _);
    }
}

pub fn tex_parameter_iv(target: types::GLenum, pname: types::GLenum, value: u32) {
    unsafe {        
        TexParameteriv(target, pname, &(value as i32) as *const i32);
    }
}

///////////
// gl 3.0
///////////
pub fn gen_vertex_array() -> u32 {
    unsafe {
        let mut arrays = [0u32; 1];
        let arrays_ptr = arrays.as_mut_ptr();
        GenVertexArrays(1, arrays_ptr);
        arrays[0]
    }
}

pub fn bind_vertex_array(array_id: u32) {
    unsafe {
        BindVertexArray(array_id);
    }
}

pub fn delete_vertex_arrays(array_ids: &[u32]) {
    unsafe {
        DeleteVertexArrays(array_ids.len() as i32, array_ids.as_ptr());
    }
}

pub fn delete_texture(tex_ids: &[u32]) {
    unsafe {
        DeleteTextures(tex_ids.len() as i32, tex_ids.as_ptr());
    }
}

pub mod helper {
    use super::*;

    pub fn enable_backface_culling() {
        enable_culling(BACK);
    }

    fn enable_culling(cull_type: types::GLenum) {        
        enable(CULL_FACE);
        cull_face(cull_type);
    }

    pub fn disable_culling() {        
        disable(CULL_FACE);        
    }

    ///////////
    // gl 4.3
    ///////////
    pub fn register_error_callback() {
        unsafe {
            enable(DEBUG_OUTPUT);
            DebugMessageCallback(error_callback, ptr::null());
        }
    }

    use std::os::raw;
    use std::ffi::CStr;

    extern "system" fn error_callback(
        _source: u32,
        gltype: u32,
        _id: u32,
        severity: u32,
        _length: i32,
        message: *const raw::c_char,
        _user_param: *mut raw::c_void,
    ) {
        if severity == DEBUG_SEVERITY_NOTIFICATION {
            return
        }
        let msg = unsafe { CStr::from_ptr(message) };
        let msg: &str = msg.to_str().unwrap();
            
        println!("GL Callback: {} type: 0x{:x}, severity: 0x{:x}, message: {}", 
            if gltype == DEBUG_TYPE_ERROR { "** GL ERROR **" } else { "" },
            gltype,
            severity,
            msg);
    }
}

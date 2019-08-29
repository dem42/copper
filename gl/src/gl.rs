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

pub fn draw_arrays_instanced(draw_type: types::GLenum, first_idx: usize, num_to_draw: usize, instancecount: usize) {
    unsafe {        
        DrawArraysInstanced(draw_type, first_idx as i32, num_to_draw as i32, instancecount as i32);
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

pub fn blend_func(sfactor: types::GLenum, dfactor: types::GLenum) {
    unsafe {
        BlendFunc(sfactor, dfactor);
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

pub fn buffer_data_unitialized<T>(target: types::GLenum, elem_count: usize, usage: types::GLenum) {
    unsafe {
        let size_in_bytes = (elem_count * mem::size_of::<T>()) as isize;
        let data_ptr = ptr::null();
        BufferData(target, size_in_bytes, data_ptr as *const _, usage);
    }
}

pub fn buffer_sub_data<T>(target: types::GLenum, offset: usize, data: &[T]) {
    unsafe {
        let size_in_bytes = (data.len() * mem::size_of::<T>()) as isize;
        let offset_size = (offset * mem::size_of::<T>()) as isize;
        BufferSubData(target, offset_size, size_in_bytes, data.as_ptr() as *const _)
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

pub fn uniform4f(location_id: i32, x: f32, y: f32, z: f32, w: f32) {
    unsafe {
        Uniform4f(location_id, x, y, z, w);
    }
}

pub fn uniform3f(location_id: i32, x: f32, y: f32, z: f32) {
    unsafe {
        Uniform3f(location_id, x, y, z);
    }
}

pub fn uniform2f(location_id: i32, x: f32, y: f32) {
    unsafe {
        Uniform2f(location_id, x, y);
    }
}

pub fn uniform_matrix4f(location_id: i32, matrix: &[[f32; 4]; 4]) {
    unsafe {        
        // hope this cast is ok .. in memory the 4x4 array should be just a block of 16 floats
        // in row major order since rust uses row major 
        // opengl matrices use same memory layout as directx which means they want the x,y,z vectors in order in memory (axes of matrix coord system)
        // and then followed by the p vector [x.x, x.y, ...., p.x, p.y, p.z, 1] in memory
        // since rust is row major where our first row is [x.x, y.x, z.x, p.x] for example we need to make sure that
        // the memory is transposed so that it has the shape that opengl expects (the one we can use to premultiply)
        let transpose = TRUE; // true implies row major order (is rust always gonna be row major or machine dependent?)
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

pub fn vertex_attrib_pointer_interleaved<T>(
    attribute_id: u32,
    components_per_attribute: u32,
    data_type: types::GLenum,
    stride: usize,
    offset: usize    
) {
    unsafe {
        let should_normalize = false as u8;
        let stride_size = (stride * mem::size_of::<T>()) as i32;
        let offset_size = (offset * mem::size_of::<T>()) as *const _; // offset to start of data in buffer
        VertexAttribPointer(
            attribute_id,
            components_per_attribute as i32,
            data_type,
            should_normalize,
            stride_size,
            offset_size,
        );
    }
}

// use this to indicate that the attribute is per instance (instanced attribute)
// the divisor says how when does the attribute's index advance .. if divisor is 0 then it advances for each vertex. non-zero means it advances each divisor instances
pub fn vertex_attrib_divisor(attrib_index: u32, divisor: u32) {
    unsafe {
        VertexAttribDivisor(attrib_index, divisor);
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

pub fn bind_texture(type_: types::GLenum, texture_id: u32) {
    unsafe {
        BindTexture(type_, texture_id);
    }
}

pub fn tex_image_2d<T>(type_: types::GLenum, level_of_detail: i32, format: types::GLenum, width: usize, height: usize, pixel_format: types::GLenum, data: &[T]) {
    unsafe {
        TexImage2D(type_, level_of_detail, format as i32, width as i32, height as i32, 0, format, pixel_format, data.as_ptr() as *const _);
    }
}

// use this to allocate memory of width * height that you can later initialize with a subtexture such as from a frame buffer attachment
pub fn tex_image_2d_uninitialized(type_: types::GLenum, level_of_detail: i32, format: types::GLenum, internal_format: types::GLenum, width: usize, height: usize, pixel_format: types::GLenum) {
    unsafe {
        TexImage2D(type_, level_of_detail, internal_format as i32, width as i32, height as i32, 0, format, pixel_format, ptr::null());
    }
}

pub fn tex_parameter_iv(target: types::GLenum, pname: types::GLenum, value: u32) {
    unsafe {        
        TexParameteriv(target, pname, &(value as i32) as *const i32);
    }
}

pub fn generate_mipmap(target: types::GLenum) {
    unsafe {        
        GenerateMipmap(target);
    }
}

// seems like the difference between TexParameteri and TexParameteriv is just that iv can accept different types of values types like border colors etc, and these 
// can take multiple parameters
pub fn tex_parameteri(target: types::GLenum, pname: types::GLenum, value: u32) {
    unsafe {        
        TexParameteri(target, pname, value as i32);
    }
}

pub fn tex_parameterf(target: types::GLenum, pname: types::GLenum, value: f32) {
    unsafe {        
        TexParameterf(target, pname, value);
    }
}

pub fn depth_mask(flag: bool) {
    unsafe {
        DepthMask(if flag { TRUE } else { FALSE });
    }
}

pub fn get_floatv(name: types::GLenum) -> f32 {
    unsafe {
        let mut result: f32 = 0.0;
        GetFloatv(name, &mut result as *mut f32);
        result
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

pub fn delete_texture(tex_id: u32) {
    unsafe {
        DeleteTextures(1, &tex_id as *const u32);
    }
}

pub fn delete_textures(tex_ids: &[u32]) {
    unsafe {
        DeleteTextures(tex_ids.len() as i32, tex_ids.as_ptr());
    }
}

pub fn gen_framebuffer() -> u32 {    
    unsafe {
        let mut fbos = [0u32; 1];
        let fbos_ptr = fbos.as_mut_ptr();
        GenFramebuffers(1, fbos_ptr);
        fbos[0]
    }
}

pub fn bind_framebuffer(fbo_type: types::GLenum, fbo_id: u32) {
    unsafe {
        BindFramebuffer(fbo_type, fbo_id);
    }
}

pub fn check_framebuffer_status(fbo: types::GLenum) {
    unsafe {
        let status = CheckFramebufferStatus(fbo);
        let status_str = status.to_string();
        if status != FRAMEBUFFER_COMPLETE {
            panic!("Error checking buffer. The buffer status is {}", match status {
                FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT",                
                FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => "GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT",
                FRAMEBUFFER_UNSUPPORTED => "GL_FRAMEBUFFER_UNSUPPORTED",
                _ => &status_str,
            }); 
        }
    }
}

pub fn draw_buffers(color_buffers: &[types::GLenum]) {
    unsafe {
        DrawBuffers(color_buffers.len() as i32, color_buffers.as_ptr());
    }
}

pub fn read_buffer(buf_type: types::GLenum) {
    unsafe {
        ReadBuffer(buf_type);
    }
}

pub fn gen_renderbuffer() -> u32 {
    unsafe {
        let mut render_bufs = [0u32; 1];
        let render_bufs_ptr = render_bufs.as_mut_ptr();
        GenRenderbuffers(1, render_bufs_ptr);
        render_bufs[0]
    }
}

pub fn bind_renderbuffer(target: types::GLenum, renderbuffer: u32) {
    unsafe {
        BindRenderbuffer(target, renderbuffer);
    }
}

pub fn renderbuffer_storage(target: types::GLenum, internalformat: types::GLenum, width: usize, height: usize) {
    unsafe {
        RenderbufferStorage(target, internalformat, width as i32, height as i32);
    }
}

pub fn framebuffer_renderbuffer(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: u32) {
    unsafe {
        FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
    }
}

pub fn delete_framebuffer(fbo_id: u32) {
    unsafe {        
        let ptr: &u32 = &fbo_id;
        DeleteFramebuffers(1, ptr as *const u32);
    }
}

pub fn delete_renderbuffer(render_buffer_id: u32) {
    unsafe {
        DeleteRenderbuffers(1, &render_buffer_id as *const u32);
    }
}

///////////
// gl 3.2
///////////
pub fn framebuffer_texture(target: types::GLenum, attachment: types::GLenum, texture: u32, level: i32) {
    unsafe {
        FramebufferTexture(target, attachment, texture, level);
    }
}

pub mod helper {
    use super::*;

    pub const CUBEMAP_FACES: [types::GLenum; 6] = [
        TEXTURE_CUBE_MAP_POSITIVE_X, TEXTURE_CUBE_MAP_NEGATIVE_X, 
        TEXTURE_CUBE_MAP_POSITIVE_Y, TEXTURE_CUBE_MAP_NEGATIVE_Y, 
        TEXTURE_CUBE_MAP_POSITIVE_Z, TEXTURE_CUBE_MAP_NEGATIVE_Z
    ];

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

    pub fn push_debug_group(id: u32, group_label: &str) {
        let group_label_nul_term = CString::new(group_label).expect("The group label must not contain 0 bytes, because we are trying to convert it to nul-term str");
        unsafe {
            // -1 means that the string is nul terminated which CStrings are
            PushDebugGroup(DEBUG_SOURCE_APPLICATION, id, -1, group_label_nul_term.as_ptr());
        }
    }

    pub fn pop_debug_group() {        
        unsafe {
            // -1 means that the string is nul terminated which CStrings are
            PopDebugGroup();
        }
    }
}

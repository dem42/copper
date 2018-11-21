include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

use std::mem;
use std::ptr;

// gl 1.1
pub fn clear_color((r, g, b, a): (f32, f32, f32, f32)) {
    unsafe {
        ClearColor(r, g, b, a);
    }
}

// gl 1.1
pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        Viewport(x, y, width, height);
    }
}

// gl 3.0
pub fn gen_vertex_array() -> u32 {
    unsafe {
        let mut arrays = [0u32; 1];
        let arrays_ptr = arrays.as_mut_ptr();
        GenVertexArrays(1, arrays_ptr);
        arrays[0]
    }
}

// gl 3.0
pub fn bind_vertex_array(array_id: u32) {
    unsafe {
        BindVertexArray(array_id);
    }
}

// gl 1.5
pub fn gen_buffer() -> u32 {
    unsafe {
        let mut buffers = [0u32; 1];
        let buffers_ptr = buffers.as_mut_ptr();
        GenBuffers(1, buffers_ptr);
        buffers[0]
    }
}

// gl 1.5
pub fn bind_buffer(kind: types::GLenum, buffer_id: u32) {
    unsafe {
        BindBuffer(kind, buffer_id);
    }
}

/// gl 1.5
/// Set data into the currently bound vbo
/// 
/// usage this tells us whether the data will be static or if we will change it later
/// 
pub fn buffer_data(target: types::GLenum, data: &[f32], usage: types::GLenum) {
    unsafe {
        let size_in_bytes = (data.len() * mem::size_of::<f32>()) as isize;
        let data_ptr = data.as_ptr();
        BufferData(target, size_in_bytes, data_ptr as *const _, usage);
    }
}

// gl 2.0
pub fn vertex_attrib_pointer(attribute_id: u32, 
                            components_per_attribute: u32, 
                            data_type: types::GLenum) {
    unsafe {
        let should_normalize = false as u8;
        let stride = 0; // dist between vertices
        let offset = ptr::null() as *const _; // offset to start of data in buffer
        VertexAttribPointer(attribute_id, components_per_attribute as i32, data_type, should_normalize, stride, offset);
    }
}

// gl 1.5
pub fn delete_buffers(buffer_ids: &[u32]) {
    unsafe {
        DeleteBuffers(buffer_ids.len() as i32, buffer_ids.as_ptr());
    }
}

// gl 3.0
pub fn delete_vertex_arrays(array_ids: &[u32]) {
    unsafe {
        DeleteVertexArrays(array_ids.len() as i32, array_ids.as_ptr());
    }
}

// gl 2.0
pub fn enable_vertex_attrib_array(attribute_id: u32) {
    unsafe {
        EnableVertexAttribArray(attribute_id);
    }
}

// gl 2.0
pub fn disable_vertex_attrib_array(attribute_id: u32) {
    unsafe {
        DisableVertexAttribArray(attribute_id);
    }
}

// gl 1.1
pub fn draw_arrays(draw_type: types::GLenum, first_idx: usize, num_to_draw: usize) {
    unsafe {
        DrawArrays(draw_type, first_idx as i32, num_to_draw as i32);
    }
}
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        Viewport(x, y, width, height);
    }
}

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

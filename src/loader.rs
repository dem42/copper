use super::gl;

#[derive(Default)]
pub struct ModelLoader {    
    vao_list: Vec<u32>,
    vbo_list: Vec<u32>,
}

impl ModelLoader {
    pub fn new() -> ModelLoader {
        // some fancy disambiguation syntax here equivalnet to Default::default() and here also to RawModel::default since no multiple functions with same name
        <ModelLoader as Default>::default()        
    }

    pub fn load_to_vao(&mut self, positions: &[f32], indices: &[u32]) -> RawModel {
        let vao_id = self.create_vao();
        let attribute_id = 0;
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(attribute_id, positions);
        self.unbind_vao();
        RawModel::new(vao_id, indices.len(), attribute_id)
    }

    fn create_vao(&mut self) -> u32 {
        let vao_id = gl::gen_vertex_array();
        self.vao_list.push(vao_id);
        gl::bind_vertex_array(vao_id);                
        vao_id
    }
    
    fn unbind_vao(&self) {
        // binding to 0 unbinds
        gl::bind_vertex_array(0);
    }
    
    fn store_data_in_attribute_list(&mut self, attribute_num: u32, data: &[f32]) {
        let vbo_id = gl::gen_buffer();
        self.vbo_list.push(vbo_id);
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo_id);
        gl::buffer_data(gl::ARRAY_BUFFER, data, gl::STATIC_DRAW);
        gl::vertex_attrib_pointer(attribute_num, 3, gl::FLOAT);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
    }

    fn bind_indices_buffer(&mut self, indices: &[u32]) {
        let vbo_id = gl::gen_buffer();
        self.vbo_list.push(vbo_id);
        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
        gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, indices, gl::STATIC_DRAW);
        // no unbind since we will bind data buffer next -> that means it HAS to be called after        
    }
}

impl Drop for ModelLoader {
    fn drop(&mut self) {
        gl::delete_vertex_arrays(&self.vao_list[..]);
        gl::delete_buffers(&self.vbo_list[..]);
    }
}

pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: usize,
    pub attribute_id: u32,
}

impl RawModel {
    pub fn new(vao_id: u32, vertex_count: usize, attribute_id: u32) -> RawModel {
        RawModel {
            vao_id,
            vertex_count,
            attribute_id,
        }
    }
}
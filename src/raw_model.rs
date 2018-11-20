use super::gl as gl;

struct RawModel {
    vaoId: u32,
    vertexCount: usize,
}

impl RawModel {
    pub fn new(vaoId: u32, vertexCount: usize) -> RawModel {
        RawModel {
            vaoId,
            vertexCount,
        }
    }

    pub fn load_to_vao(positions: &[f32]) -> RawModel {
        let vao_id = RawModel::create_vao();
        RawModel::store_data_in_attribute_list(0, positions);
        RawModel::unbind_vao();
        RawModel::new(vao_id, positions.len() / 3)
    }

    fn create_vao() -> u32 {
        let vao_id = gl::gen_vertex_array();
        gl::bind_vertex_array(vao_id);
        vao_id
    }


    fn store_data_in_attribute_list(attribute_num: i32, data: &[f32]) {
        let vbo_id = gl::gen_buffer();
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo_id);
        unimplemented!()
    }

    fn unbind_vao() {
        // binding to 0 unbinds
        gl::bind_vertex_array(0);
    }
}
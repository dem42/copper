struct RawModel {
    vaoId: i32,
    vertexCount: usize,
}

impl RawModel {
    pub fn new(vaoId: i32, vertexCount: usize) -> RawModel {
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

    fn create_vao() -> i32 {
        unimplemented!()
        // unsafe {
        //     let vao_id = glfw_sys::gl
        // }
    }

    fn store_data_in_attribute_list(attribute_num: i32, data: &[f32]) {
        unimplemented!()
    }

    fn unbind_vao() {
        unimplemented!()
    }
}
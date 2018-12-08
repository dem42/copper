use super::gl;
use texture_lib::texture_loader::{
    load_rgb_2d_texture
};

#[derive(Default)]
pub struct ModelLoader {    
    vao_list: Vec<u32>,
    vbo_list: Vec<u32>,
    tex_list: Vec<u32>,
}

impl ModelLoader {
    pub fn new() -> ModelLoader {
        // some fancy disambiguation syntax here equivalnet to Default::default() and here also to RawModel::default since no multiple functions with same name
        <ModelLoader as Default>::default()        
    }

    pub fn load_to_vao(&mut self, positions: &[f32], texture_coords: &[f32], indices: &[u32], normals: &[f32]) -> RawModel {
        let vao_id = self.create_vao();
        let pos_attrib = 0;
        let tex_coord_attrib = 1;
        let normal_attrib = 2;
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(pos_attrib, 3, positions);
        self.store_data_in_attribute_list(tex_coord_attrib, 2, texture_coords);
        self.store_data_in_attribute_list(normal_attrib, 3, normals);
        self.unbind_vao();
        RawModel::new(vao_id, indices.len(), pos_attrib, tex_coord_attrib, normal_attrib)
    }

    pub fn load_texture(&mut self, file_name: &str, shine_damper: f32, reflectivity: f32, reverse: bool) -> ModelTexture {
        let texture = load_rgb_2d_texture(file_name, reverse).expect("Failed to load texture");
        println!("got rgb vec with {} elements", texture);

        let tex_id = gl::gen_texture();
        self.tex_list.push(tex_id);
        gl::bind_texture(tex_id, gl::TEXTURE_2D);

        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);

        gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGB, texture.width, texture.height, gl::UNSIGNED_BYTE, &texture.data);
        gl::bind_texture(0, gl::TEXTURE_2D);
        ModelTexture {
            tex_id,
            shine_damper,
            reflectivity,
        }
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
    
    fn store_data_in_attribute_list(&mut self, attribute_num: u32, coord_size: u32, data: &[f32]) {
        let vbo_id = gl::gen_buffer();
        self.vbo_list.push(vbo_id);
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo_id);
        gl::buffer_data(gl::ARRAY_BUFFER, data, gl::STATIC_DRAW);
        gl::vertex_attrib_pointer(attribute_num, coord_size, gl::FLOAT);
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
        gl::delete_texture(&self.tex_list);
    }
}

pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: usize,
    pub pos_attrib: u32,
    pub tex_coord_attrib: u32,
    pub normal_attrib: u32,
}

impl RawModel {
    pub fn new(vao_id: u32, vertex_count: usize, pos_attrib: u32, tex_coord_attrib: u32, normal_attrib: u32) -> RawModel {
        RawModel {
            vao_id,
            vertex_count,
            pos_attrib,
            tex_coord_attrib,
            normal_attrib,
        }
    }
}

pub struct ModelTexture {
    pub tex_id: u32,
    pub shine_damper: f32,
    pub reflectivity: f32,
}

impl Default for ModelTexture {
    fn default() -> ModelTexture {
        ModelTexture {
            tex_id: 0,
            shine_damper: 1.0,
            reflectivity: 0.0,
        }
    }
}

pub struct TexturedModel {
    pub raw_model: RawModel,
    pub texture: ModelTexture,
}
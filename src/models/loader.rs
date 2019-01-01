use crate::gl;
use texture_lib::texture_loader::{
    load_rgba_2d_texture
};
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub struct ModelLoader {    
    vao_list: Vec<u32>,
    vbo_list: Vec<u32>,
    tex_list: Vec<u32>,
}

#[repr(u8)]
pub enum TextureFlags {
    REVERSE = 1,
    MIPMAP = 2,
}

impl TextureFlags {
    fn parse(flags_mask: u8) -> (bool, bool) {        
        let is_reverse = (flags_mask & (TextureFlags::REVERSE as u8)) == (TextureFlags::REVERSE as u8);
        let uses_mipmaps = (flags_mask & (TextureFlags::MIPMAP as u8)) == (TextureFlags::MIPMAP as u8);
        (is_reverse, uses_mipmaps)
    }
}

impl ModelLoader {
    pub fn new() -> ModelLoader {
        // some fancy disambiguation syntax here equivalnet to Default::default() and here also to RawModel::default since no multiple functions with same name
        <ModelLoader as Default>::default()        
    }

    pub fn load_to_vao(&mut self, positions: &[f32], texture_coords: &[f32], indices: &[u32], normals: &[f32]) -> RawModel {
        let vao_id = self.create_vao();
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(RawModel::POS_ATTRIB, 3, positions);
        self.store_data_in_attribute_list(RawModel::TEX_COORD_ATTRIB, 2, texture_coords);
        self.store_data_in_attribute_list(RawModel::NORMAL_ATTRIB, 3, normals);
        self.unbind_vao();
        RawModel::new(vao_id, indices.len())
    }

    pub fn load_texture(&mut self, file_name: &str, flags: u8) -> ModelTexture {
        let (reverse, mipmap) = TextureFlags::parse(flags);
        let texture = load_rgba_2d_texture(file_name, reverse).expect(&format!("Failed to load texture: {}", file_name));
        
        let tex_id = gl::gen_texture();
        self.tex_list.push(tex_id);
        gl::bind_texture(tex_id, gl::TEXTURE_2D);

        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);        

        gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGBA, texture.width, texture.height, gl::UNSIGNED_BYTE, &texture.data);
        if mipmap {
             // turn on mipmapping, has to be called after loading the texture data 
            gl::generate_mipmap(gl::TEXTURE_2D);
            gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
            // set texture detail level (more negative means nicer) things at a high angle like grass/flowers may seem blurry if this is positive or 0
            gl::tex_parameterf(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, -0.4);
        } else {        
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        }

        gl::bind_texture(0, gl::TEXTURE_2D);
        ModelTexture {
            tex_id,
            ..Default::default()
        }
    }

    pub fn load_terrain_texture(&mut self, file_name: &str, flags: u8) -> TerrainTexture {
        let (reverse, mipmap) = TextureFlags::parse(flags);
        let texture = load_rgba_2d_texture(file_name, reverse).expect(&format!("Failed to load terrain texture: {}", file_name));
        
        let tex_id = gl::gen_texture();
        self.tex_list.push(tex_id);
        gl::bind_texture(tex_id, gl::TEXTURE_2D);

        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);        
        gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGBA, texture.width, texture.height, gl::UNSIGNED_BYTE, &texture.data);
        if mipmap {
             // turn on mipmapping
            gl::generate_mipmap(gl::TEXTURE_2D);
            gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
            // set texture detail level (more negative means nicer) things at a high angle like grass/flowers may seem blurry if this is positive or 0
            gl::tex_parameterf(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, -0.4);
        } else {        
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        }

        gl::bind_texture(0, gl::TEXTURE_2D);
        TerrainTexture {
            tex_id,
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
}

impl RawModel {
    pub const POS_ATTRIB: u32 = 0;
    pub const TEX_COORD_ATTRIB: u32 = 1;
    pub const NORMAL_ATTRIB: u32 = 2;

    pub fn new(vao_id: u32, vertex_count: usize) -> RawModel {
        RawModel {
            vao_id,
            vertex_count,
        }
    }
}

pub struct TerrainTexture {
    pub tex_id: u32,
}

pub struct TerrainTexturePack {
    pub background_texture: TerrainTexture,
    pub r_texture: TerrainTexture,
    pub g_texture: TerrainTexture,
    pub b_texture: TerrainTexture,
}

pub struct ModelTexture {
    pub tex_id: u32,
    pub shine_damper: f32,
    pub reflectivity: f32,
    pub has_transparency: bool,
    pub uses_fake_lighting: bool,
}

impl Default for ModelTexture {
    fn default() -> ModelTexture {
        ModelTexture {
            tex_id: 0,
            shine_damper: 1.0,
            reflectivity: 0.0,
            has_transparency: false,
            uses_fake_lighting: false,
        }
    }
}

pub struct TexturedModel {
    pub raw_model: RawModel,
    pub texture: ModelTexture,
}

impl PartialEq for TexturedModel {
    fn eq(&self, other: &TexturedModel) -> bool {
        self.texture.tex_id == other.texture.tex_id && self.raw_model.vao_id == other.raw_model.vao_id
    }
}

impl Eq for TexturedModel {}

impl Hash for TexturedModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.texture.tex_id.hash(state);
        self.raw_model.vao_id.hash(state);
    }
}
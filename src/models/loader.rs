use crate::gl;
use texture_lib::texture_loader::{
    load_rgba_2d_texture,
};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Default)]
pub struct ModelLoader {    
    vao_list: Vec<u32>,
    vbo_list: Vec<u32>,
    tex_list: Vec<u32>,
}

#[derive(Default)]
pub struct TextureParams {
    reverse_texture_data: bool,
    use_mipmap: bool,
    mipmap_lod: f32,
}

impl TextureParams {    
    pub fn mipmapped_texture(mipmap_lod: f32) -> TextureParams {
        TextureParams {
            use_mipmap: true,
            mipmap_lod,
            ..Default::default()
        }
    }
}

impl ModelLoader {
    pub fn new() -> ModelLoader {
        // some fancy disambiguation syntax here equivalnet to Default::default() and here also to RawModel::default since no multiple functions with same name
        <ModelLoader as Default>::default()        
    }

    pub fn load_to_vao_with_normal_map(&mut self, positions: &[f32], texture_coords: &[f32], indices: &[u32], normals: &[f32], tangents: &[f32]) -> RawModel {
        let vao_id = self.create_vao();
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(RawModel::POS_ATTRIB, 3, positions);
        self.store_data_in_attribute_list(RawModel::TEX_COORD_ATTRIB, 2, texture_coords);
        self.store_data_in_attribute_list(RawModel::NORMAL_ATTRIB, 3, normals);
        self.store_data_in_attribute_list(RawModel::TANGENT_ATTRIB, 4, tangents);
        self.unbind_vao();
        RawModel::new(vao_id, indices.len())
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

    pub fn load_simple_model_to_vao(&mut self, positions: &[f32], dimension: u32) -> RawModel {
        let vao_id = self.create_vao();        
        self.store_data_in_attribute_list(RawModel::POS_ATTRIB, dimension, positions);        
        self.unbind_vao();
        RawModel::new(vao_id, positions.len() / 2)
    }

    pub fn load_dynamic_model_with_indices_to_vao(&mut self, unique_vertex_count: usize, indices: &[u32], dimension: u32) -> DynamicVertexIndexedModel {
        let vao_id = self.create_vao(); 
        self.bind_indices_buffer(indices);     
        let stream_draw_vbo = self.create_empty_float_vbo_for_attrib(RawModel::POS_ATTRIB, unique_vertex_count, dimension);
        self.unbind_vao();
        DynamicVertexIndexedModel {
            raw_model: RawModel::new(vao_id, indices.len()),
            stream_draw_vbo,
        }
    }

    pub fn load_quads_mesh_to_vao(&mut self, positions: &[f32], texture_coords: &[f32]) -> RawModel {
        let vao_id = self.create_vao(); 
        self.store_data_in_attribute_list(RawModel::POS_ATTRIB, 2, positions);        
        self.store_data_in_attribute_list(RawModel::TEX_COORD_ATTRIB, 2, texture_coords);   
        self.unbind_vao();
        RawModel::new(vao_id, positions.len() / 2)
    }

    pub fn load_cube_map(&mut self, cube_map_folder: &str) -> u32 {        
        let cubemap_id = gl::gen_texture();
        self.tex_list.push(cubemap_id);
        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_CUBE_MAP, cubemap_id);

        for i in 1..=6 {
            let filename = format!("{}/{}.png", cube_map_folder, i);
            let texture = load_rgba_2d_texture(&filename, false).expect(&format!("Failed to load texture: {}", &filename));

            gl::tex_image_2d(gl::helper::CUBEMAP_FACES[i-1], 0, gl::RGBA, texture.width, texture.height, gl::UNSIGNED_BYTE, &texture.data);

            gl::tex_parameter_iv(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::tex_parameter_iv(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
            gl::tex_parameter_iv(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl::tex_parameter_iv(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);            
        }
        gl::bind_texture(gl::TEXTURE_CUBE_MAP, 0);
        cubemap_id
    }

    fn load_texture_internal(&mut self, file_name: &str, params: TextureParams) -> u32 {
        let texture = load_rgba_2d_texture(file_name, params.reverse_texture_data).expect(&format!("Failed to load texture: {}", file_name));
        
        let tex_id = gl::gen_texture();
        self.tex_list.push(tex_id);
        gl::active_texture(gl::TEXTURE0); // even though 0 is default i think, just to be explicit let's activate texture unit 0
        gl::bind_texture(gl::TEXTURE_2D, tex_id);

        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT);        

        gl::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGBA, texture.width, texture.height, gl::UNSIGNED_BYTE, &texture.data);
        if params.use_mipmap {
             // turn on mipmapping, has to be called after loading the texture data 
            gl::generate_mipmap(gl::TEXTURE_2D);
            gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
            // set texture detail level (more negative means nicer) things at a high angle like grass/flowers may seem blurry if this is positive or 0
            gl::tex_parameterf(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, params.mipmap_lod);
        } else {        
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        }

        gl::bind_texture(gl::TEXTURE_2D, 0);        
        tex_id        
    }

     pub fn load_gui_texture(&mut self, file_name: &str, params: TextureParams) -> u32 {
        self.load_texture_internal(file_name, params)
     }

    pub fn load_texture(&mut self, file_name: &str, params: TextureParams) -> ModelTexture {        
        ModelTexture {
            tex_id: self.load_texture_internal(file_name, params),
            ..Default::default()
        }
    }

    pub fn load_particle_texture(&mut self, file_name: &str, params: TextureParams) -> ParticleTexture {        
        ParticleTexture {
            tex_id: self.load_texture_internal(file_name, params),
            ..Default::default()
        }
    }

    pub fn load_terrain_texture(&mut self, file_name: &str, params: TextureParams) -> TerrainTexture {        
        TerrainTexture {
            tex_id: self.load_texture_internal(file_name, params),
        }
    }

    pub fn create_empty_float_vbo(&mut self, float_count: usize) -> u32 {
        let vbo_id = gl::gen_buffer();
        self.vbo_list.push(vbo_id);
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo_id);
        gl::buffer_data_unitialized::<f32>(gl::ARRAY_BUFFER, float_count, gl::STREAM_DRAW);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
        vbo_id
    }

    pub fn create_empty_float_vbo_for_attrib(&mut self, attribute_num: u32, item_count: usize, coord_size: u32) -> u32 {
        let vbo_id = gl::gen_buffer();
        self.vbo_list.push(vbo_id);
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo_id);
        gl::buffer_data_unitialized::<f32>(gl::ARRAY_BUFFER, item_count * (coord_size as usize), gl::STREAM_DRAW);
        gl::vertex_attrib_pointer(attribute_num, coord_size, gl::FLOAT);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
        vbo_id
    }

    pub fn add_instanced_attrib(&mut self, vao: u32, vbo: u32, attrib: u32, components_per_attribute: u32, instanced_data_length: usize, offset: usize) {
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
        gl::bind_vertex_array(vao);
        gl::vertex_attrib_pointer_interleaved::<f32>(attrib, components_per_attribute, gl::FLOAT, instanced_data_length, offset);
        gl::vertex_attrib_divisor(attrib, 1);
        gl::bind_vertex_array(0);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
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
        gl::delete_textures(&self.tex_list);
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct RawModel {
    pub vao_id: u32,
    pub vertex_count: usize,
}

impl RawModel {
    pub const POS_ATTRIB: u32 = 0;
    pub const TEX_COORD_ATTRIB: u32 = 1;
    pub const NORMAL_ATTRIB: u32 = 2;
    pub const TANGENT_ATTRIB: u32 = 3;

    pub fn new(vao_id: u32, vertex_count: usize) -> RawModel {
        RawModel {
            vao_id,
            vertex_count,
        }
    }
}

#[derive(Clone)]
pub struct TerrainTexture {
    pub tex_id: u32,
}

#[derive(Clone)]
pub struct TerrainTexturePack {
    pub background_texture: TerrainTexture,
    pub r_texture: TerrainTexture,
    pub g_texture: TerrainTexture,
    pub b_texture: TerrainTexture,
}

#[derive(Clone)]
pub struct ModelTexture {
    pub tex_id: u32,
    pub shine_damper: f32,
    pub reflectivity: f32,
    pub has_transparency: bool,
    pub uses_fake_lighting: bool,
    // if this is 1 then the texture is not an atlas
    // also rows == columns since textures are power of two squares and so are textures
    pub number_of_rows_in_atlas: usize,
}

impl Default for ModelTexture {
    fn default() -> ModelTexture {
        ModelTexture {
            tex_id: 0,
            shine_damper: 1.0,
            reflectivity: 0.0,
            has_transparency: false,
            uses_fake_lighting: false,
            number_of_rows_in_atlas: 1,
        }
    }
}

#[derive(Clone)]
pub struct TexturedModel {
    pub raw_model: RawModel,
    pub texture: ModelTexture,
    pub normal_map_tex_id: Option<u32>,
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

#[derive(Clone)]
pub struct TerrainModel {
    pub raw_model: RawModel,
    pub height_map: Rc<Vec<Vec<f32>>>,
}

#[derive(Clone)]
pub struct GuiModel {
    pub raw_model: RawModel,
}

#[derive(Clone)]
pub struct SkyboxModel {
    pub raw_model: RawModel,
    pub day_texture_id: u32,
    pub night_texture_id: u32,
}

#[derive(Clone)]
pub struct WaterModel {
    pub raw_model: RawModel,
    pub dudv_tex_id: u32,
    pub normal_map_tex_id: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ParticleModel {
    pub raw_model: RawModel,
    pub stream_draw_vbo: u32,
}

#[derive(Clone)]
pub struct DynamicVertexIndexedModel {
    pub raw_model: RawModel,
    pub stream_draw_vbo: u32,
}

impl ParticleModel {    
    pub const MODELVIEW_COLUMN1: u32 = 1;    
    pub const MODELVIEW_COLUMN2: u32 = 2;    
    pub const MODELVIEW_COLUMN3: u32 = 3;    
    pub const MODELVIEW_COLUMN4: u32 = 4;    
    pub const TEX_OFFSET: u32 = 5;    
    pub const BLEND: u32 = 6;
    
    // 21 = (4 + 4 + 4 + 4) + 4 + 1 which is how many floats the shader needs
    pub const INSTANCED_DATA_LENGTH: usize = 21;
    pub const MAX_INSTANCES: usize = 10_000;
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ParticleTexture {
    pub tex_id: u32,
    pub number_of_rows_in_atlas: usize,
    pub additive: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ParticleTexturedModel {
    pub model: ParticleModel,
    pub texture: ParticleTexture,
}
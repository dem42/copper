use crate::models::{
    RawModel,
    TerrainTexture,
	TerrainTexturePack,
    ModelLoader,
};

pub struct Terrain<'a> {
    pub x: f32,
    pub z: f32,
    pub raw_model: &'a RawModel,
    pub blend_texture: &'a TerrainTexture,
    pub texture_pack: &'a TerrainTexturePack,
}

impl<'a> Terrain<'a> {
    const SIZE: f32 = 800.0;
    const VERTEX_COUNT: usize = 128;

    pub fn new(grid_x: i32, grid_z: i32, texture_pack: &'a TerrainTexturePack, blend_texture: &'a TerrainTexture, terrain_model: &'a RawModel) -> Terrain<'a> {        
        Terrain {
            x: grid_x as f32 * Terrain::SIZE,
            z: grid_z as f32 * Terrain::SIZE,
            blend_texture,
            raw_model: terrain_model,
			texture_pack,
        }
    }
    
    pub fn generate_terrain(loader: &mut ModelLoader) -> RawModel {
		const COUNT: usize = Terrain::VERTEX_COUNT * Terrain::VERTEX_COUNT;
		let mut vertices = [0.0f32; COUNT * 3];
		let mut normals = [0.0f32; COUNT * 3];
		let mut texture_coords = [0.0f32; COUNT * 2];
		let mut indices = [0u32; 6*(Terrain::VERTEX_COUNT-1)*(Terrain::VERTEX_COUNT-1)];
		let mut vertex_pointer = 0;
		for i in 0..Terrain::VERTEX_COUNT {
			for j in 0..Terrain::VERTEX_COUNT {
				vertices[vertex_pointer*3] = (j as f32/(Terrain::VERTEX_COUNT - 1) as f32) * Terrain::SIZE;
				vertices[vertex_pointer*3+1] = 0.0;
				vertices[vertex_pointer*3+2] = (i as f32/(Terrain::VERTEX_COUNT - 1) as f32) * Terrain::SIZE;
				normals[vertex_pointer*3] = 0.0;
				normals[vertex_pointer*3+1] = 1.0;
				normals[vertex_pointer*3+2] = 0.0;
				texture_coords[vertex_pointer*2] = j as f32/(Terrain::VERTEX_COUNT - 1) as f32;
				texture_coords[vertex_pointer*2+1] = i as f32/(Terrain::VERTEX_COUNT - 1) as f32;
				vertex_pointer+=1;
			}
		}
		let mut pointer = 0;
		for gz in 0..Terrain::VERTEX_COUNT-1 {
			for gx in 0..Terrain::VERTEX_COUNT-1 {
				let top_left = (gz*Terrain::VERTEX_COUNT)+gx;
				let top_right = top_left + 1;
				let bottom_left = ((gz+1)*Terrain::VERTEX_COUNT)+gx;
				let bottom_right = bottom_left + 1;
				indices[pointer] = top_left as u32;
                pointer+=1;
				indices[pointer] = bottom_left as u32;
                pointer+=1;
				indices[pointer] = top_right as u32;
                pointer+=1;
				indices[pointer] = top_right as u32;
                pointer+=1;
				indices[pointer] = bottom_left as u32;
                pointer+=1;
				indices[pointer] = bottom_right as u32;
                pointer+=1;
			}
		}
		loader.load_to_vao(&vertices, &texture_coords, &indices, &normals)
	}
}
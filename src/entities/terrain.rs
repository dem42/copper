use std::rc::Rc;
use crate::models::{
    TerrainTexture,
	TerrainTexturePack,
    ModelLoader,
	TerrainModel,
	TerrainGenerator,	
};

pub struct Terrain {
    pub x: f32,
    pub z: f32,
    pub model: TerrainModel,
    pub blend_texture: TerrainTexture,
    pub texture_pack: TerrainTexturePack,
}

impl Terrain {
    pub const SIZE: f32 = 800.0;
    
    pub fn new(grid_x: i32, grid_z: i32, texture_pack: TerrainTexturePack, blend_texture: TerrainTexture, terrain_model: TerrainModel) -> Terrain {        
        Terrain {
            x: grid_x as f32 * Terrain::SIZE,
            z: grid_z as f32 * Terrain::SIZE,
            blend_texture,
            model: terrain_model,
			texture_pack,
        }
    }

	pub fn is_xz_within_terrain_cell(&self, x: f32, z: f32) -> bool {
		self.x <= x && x < self.x + Terrain::SIZE && self.z <= z && z < self.z + Terrain::SIZE 
	}
	    
    pub fn generate_terrain(loader: &mut ModelLoader, terrain_generator: &TerrainGenerator) -> TerrainModel {
		let vertex_count: usize = terrain_generator.width();	
		let count: usize = vertex_count * vertex_count;
		let mut height_array = vec![vec![0.0f32; vertex_count]; vertex_count];

		let mut vertices = vec![0.0f32; count * 3];
		let mut normals = vec![0.0f32; count * 3];
		let mut texture_coords = vec![0.0f32; count * 2];
		let mut indices = vec![0u32; 6*(vertex_count-1)*(vertex_count-1)];
		let mut vertex_pointer = 0;
		for i in 0..vertex_count {
			for j in 0..vertex_count {
				let height_at_xz = terrain_generator.get_height(j as isize, i as isize);
				height_array[j][i] = height_at_xz;
				vertices[vertex_pointer*3] = (j as f32/(vertex_count - 1) as f32) * Terrain::SIZE;				
				vertices[vertex_pointer*3+1] = height_at_xz;
				vertices[vertex_pointer*3+2] = (i as f32/(vertex_count - 1) as f32) * Terrain::SIZE;
				let normal = terrain_generator.get_normal_at(j as isize, i as isize);
				normals[vertex_pointer*3] = normal.x;
				normals[vertex_pointer*3+1] = normal.y;
				normals[vertex_pointer*3+2] = normal.z;
				texture_coords[vertex_pointer*2] = j as f32/(vertex_count - 1) as f32;
				texture_coords[vertex_pointer*2+1] = i as f32/(vertex_count - 1) as f32;
				vertex_pointer+=1;
			}
		}
		let mut pointer = 0;
		for gz in 0..vertex_count-1 {
			for gx in 0..vertex_count-1 {
				let top_left = (gz*vertex_count)+gx;
				let top_right = top_left + 1;
				let bottom_left = ((gz+1)*vertex_count)+gx;
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
		TerrainModel {
			raw_model: loader.load_to_vao(&vertices, &texture_coords, &indices, &normals),
			height_map: Rc::new(height_array),
		}
	}
}
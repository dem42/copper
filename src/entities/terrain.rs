use crate::models::{
    TerrainTexture,
	TerrainTexturePack,
    ModelLoader,
	TerrainModel,
};
use crate::math::{
	Vector3f,
};
use texture_lib::texture_loader::{
	load_rgb_2d_texture,
	Texture2DRGB,
};

pub struct Terrain<'a> {
    pub x: f32,
    pub z: f32,
    pub model: &'a TerrainModel,
    pub blend_texture: &'a TerrainTexture,
    pub texture_pack: &'a TerrainTexturePack,
}

impl<'a> Terrain<'a> {
    pub const SIZE: f32 = 800.0;
    const MAX_HEIGHT: f32 = 40.0;
    
    pub fn new(grid_x: i32, grid_z: i32, texture_pack: &'a TerrainTexturePack, blend_texture: &'a TerrainTexture, terrain_model: &'a TerrainModel) -> Terrain<'a> {        
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

	fn get_height_from_heightmap(height_map: &Texture2DRGB, x: isize, z: isize) -> f32 {
		if x < 0 || x >= height_map.height as isize || z < 0 || z >= height_map.width as isize {
			return 0.0;
		}
		let (x, z) = (x as usize, z as usize);
		let color = height_map.get_color(z, x);		
		let grayscale = color.r as f32;
		const MAX_COLOR: f32 = 256.0;
		let rescaled = (grayscale - MAX_COLOR/2.0) / MAX_COLOR;
		rescaled * Terrain::MAX_HEIGHT
	}

	// fn compute_terrain_grid_normal_faster(height_map: &Texture2DRGB, i: isize, j: isize) -> Vector3f {
	// 	let lh = Terrain::get_height_from_heightmap(height_map, i, j-1);
	// 	let rh = Terrain::get_height_from_heightmap(height_map, i, j+1);
	// 	let uh = Terrain::get_height_from_heightmap(height_map, i-1, j);
	// 	let dh = Terrain::get_height_from_heightmap(height_map, i+1, j);
	// 	let mut normal = Vector3f::new(lh - rh, 2.0, dh - uh);
	// 	normal.normalize();
	// 	normal
	// }

	fn compute_terrain_grid_normal(height_map: &Texture2DRGB, x: isize, z: isize) -> Vector3f {
		let lh = Terrain::get_height_from_heightmap(height_map, x-1, z);
		let rh = Terrain::get_height_from_heightmap(height_map, x+1, z);
		let uh = Terrain::get_height_from_heightmap(height_map, x, z-1);
		let dh = Terrain::get_height_from_heightmap(height_map, x, z+1);
		let x_dir_tangent = Vector3f::new(2.0, rh - lh, 0.0); // the 2 is from z+1 - (z-1)
		let z_dir_tangent = Vector3f::new(0.0, dh - uh, 2.0);
		// in RHS in k x i = j so since i am doing i x k to get positive j direction i take -
		let mut cross_prod = -x_dir_tangent.cross_prod(&z_dir_tangent);
		cross_prod.normalize();
		cross_prod
	}
    
    pub fn generate_terrain(loader: &mut ModelLoader, height_map: &str) -> TerrainModel {
		let height_data = load_rgb_2d_texture(height_map, false).expect(&format!("Couldn't load height map file: {}", height_map));
		assert_eq!(height_data.width, height_data.height, "Height map must be square");	

		let vertex_count: usize = height_data.width;	
		let count: usize = vertex_count * vertex_count;
		let mut height_array = vec![vec![0.0f32; vertex_count]; vertex_count];

		let mut vertices = vec![0.0f32; count * 3];
		let mut normals = vec![0.0f32; count * 3];
		let mut texture_coords = vec![0.0f32; count * 2];
		let mut indices = vec![0u32; 6*(vertex_count-1)*(vertex_count-1)];
		let mut vertex_pointer = 0;
		for i in 0..vertex_count {
			for j in 0..vertex_count {
				let height_at_xz = Terrain::get_height_from_heightmap(&height_data, j as isize, i as isize);
				height_array[j][i] = height_at_xz;
				vertices[vertex_pointer*3] = (j as f32/(vertex_count - 1) as f32) * Terrain::SIZE;				
				vertices[vertex_pointer*3+1] = height_at_xz;
				vertices[vertex_pointer*3+2] = (i as f32/(vertex_count - 1) as f32) * Terrain::SIZE;
				let normal = Terrain::compute_terrain_grid_normal(&height_data, j as isize, i as isize);
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
			height_map: height_array,
		}
	}
}
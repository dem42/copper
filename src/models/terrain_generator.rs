use std::f32;
use crate::math::Vector3f;
use texture_lib::texture_loader::{
	load_rgba_2d_texture,
	Texture2DRGBA,
};
use crate::utils::gen_murmur3_f32;

pub trait TerrainGenerator {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn get_height(&self, x: isize, z: isize) -> f32;

	// fn compute_terrain_grid_normal_faster(height_map: &Texture2DRGB, i: isize, j: isize) -> Vector3f {
	// 	let lh = Terrain::get_height_from_heightmap(height_map, i, j-1);
	// 	let rh = Terrain::get_height_from_heightmap(height_map, i, j+1);
	// 	let uh = Terrain::get_height_from_heightmap(height_map, i-1, j);
	// 	let dh = Terrain::get_height_from_heightmap(height_map, i+1, j);
	// 	let mut normal = Vector3f::new(lh - rh, 2.0, dh - uh);
	// 	normal.normalize();
	// 	normal
	// }

    fn get_normal_at(&self, x: isize, z: isize) -> Vector3f {
		let lh = self.get_height(x-1, z);
		let rh = self.get_height(x+1, z);
		let uh = self.get_height(x, z-1);
		let dh = self.get_height(x, z+1);
		let x_dir_tangent = Vector3f::new(2.0, rh - lh, 0.0); // the 2 is from z+1 - (z-1)
		let z_dir_tangent = Vector3f::new(0.0, dh - uh, 2.0);
		// in RHS in k x i = j so since i am doing i x k to get positive j direction i take -
		let mut cross_prod = -x_dir_tangent.cross_prod(&z_dir_tangent);
		cross_prod.normalize();
		cross_prod
	}    
}

pub struct HeightMap {
    height_map: Texture2DRGBA, 
}

impl HeightMap {
    const MAX_HEIGHT: f32 = 40.0;

    pub fn new(height_map: &str) -> Self {
        let height_data = load_rgba_2d_texture(height_map, false).expect(&format!("Couldn't load height map file: {}", height_map));
		assert_eq!(height_data.width, height_data.height, "Height map must be square");	
        HeightMap {
            height_map: height_data,
        }
    }
}

impl TerrainGenerator for HeightMap {
    fn height(&self) -> usize {
        self.height_map.height
    }

    fn width(&self) -> usize {
        self.height_map.width
    }

    fn get_height(&self, x: isize, z: isize) -> f32 {
		if x < 0 || x >= self.height() as isize || z < 0 || z >= self.width() as isize {
			return 0.0;
		}
		let (x, z) = (x as usize, z as usize);
		let color = self.height_map.get_color(z, x);		
		let grayscale = color.r as f32;
		const MAX_COLOR: f32 = 256.0;
		let rescaled = (grayscale - MAX_COLOR/2.0) / MAX_COLOR;
		rescaled * HeightMap::MAX_HEIGHT
	}
}

pub struct HeightsGenerator {
    seed: u32,
}

impl HeightsGenerator {    
    const HEIGHT: usize = 128;
    const WIDTH: usize = 128;
    const AMPLITUDE: f32 = 70.0;
    // how many different frequency noises do we want to combine
    const OCTAVES: usize = 3;
    const ROUGHNESS: f32 = 0.3;

    pub fn get_smooth_noise(&self, x: isize, y: isize) -> f32 {
        // we do gaussian blurring here to smoothen the noise
        // to do so we use a 3x3 gaussian blur which approximates the weights 
        // that would be obtained from a sampling of normal distribution
        let corners = self.get_noise(x-1,y-1) + self.get_noise(x-1,y+1) + self.get_noise(x+1,y-1) + self.get_noise(x+1,y+1);
        let edges = 2.0 * (self.get_noise(x-1,y) + self.get_noise(x+1,y) + self.get_noise(x,y-1) + self.get_noise(x,y+1));
        let center = 4.0 * self.get_noise(x, y);
        (center + edges + corners) / 16.0
    }

    fn get_noise(&self, x: isize, y: isize) -> f32 {
        let hash = gen_murmur3_f32(x as u32, y as u32, self.seed);        
        2.0 * hash - 1.0
    }

    fn get_interpolated_noise(&self, x: f32, y: f32) -> f32 {
        let x_whole = x.floor();
        let y_whole = y.floor();
        let x_frac = x - x_whole;
        let y_frac = y - y_whole;
        let x_whole = x_whole as isize;
        let y_whole = y_whole as isize;
        let noise_1 = self.get_smooth_noise(x_whole, y_whole);
        let noise_2 = self.get_smooth_noise(x_whole+1, y_whole);
        let noise_3 = self.get_smooth_noise(x_whole, y_whole+1);
        let noise_4 = self.get_smooth_noise(x_whole+1, y_whole+1);
        let noise_mid_12 = HeightsGenerator::cosine_interpolate(noise_1, noise_2, x_frac);
        let noise_mid_34 = HeightsGenerator::cosine_interpolate(noise_3, noise_4, x_frac);
        let noise_mid = HeightsGenerator::cosine_interpolate(noise_mid_12, noise_mid_34, y_frac);
        noise_mid
    }

    fn cosine_interpolate(x0: f32, x1: f32, alpha: f32) -> f32 {
        let angle = alpha * f32::consts::PI;
        // our angle is [0,180] degs and that maps to cos [1,-1] which we want to map to [0,1] using the following
        let cosine_alpha = (1.0 - angle.cos()) * 0.5;
        x0 * (1.0 - cosine_alpha) + cosine_alpha * x1
    }
}

impl Default for HeightsGenerator {
    fn default() -> Self {     
        let seed = 0x00c0ffee;
        HeightsGenerator {
            seed,
        }    
    }
}

impl TerrainGenerator for HeightsGenerator {    
    fn height(&self) -> usize {
        HeightsGenerator::HEIGHT
    }

    fn width(&self) -> usize {
        HeightsGenerator::WIDTH
    }

    fn get_height(&self, x: isize, z: isize) -> f32 {
        // our interpolated noise has cosine splines between procedurally generated noise peaks
        // if we sample from a narrow region the noise is smoother (low frequency), since the cosine splines are very smooth
        // if we sample from a wide region the noise is spiky (high frequency) since we get less of the splines and more of the spiky noisy peaks
        let x = x as f32;
        let z = z as f32;
        let mut total_noise = 0.0;
        let mut octave_width = 1 << (HeightsGenerator::OCTAVES - 1);
        let mut amplitude = HeightsGenerator::AMPLITUDE; 
        for _ in 0..HeightsGenerator::OCTAVES {            
            let octave_width_f32 = octave_width as f32;
            total_noise += self.get_interpolated_noise(x / octave_width_f32, z / octave_width_f32) * amplitude;
            octave_width = octave_width >> 1;
            amplitude *= HeightsGenerator::ROUGHNESS;
        }        
        total_noise
    }
}
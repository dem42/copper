use crate::entities::Terrain;
use crate::math::{
    Vector3f,
    Vector2f,
    BarycentricCoords,
};
use std::f32;
use std::cmp;

pub struct Ground<'a> {
    pub terrains: Vec<Terrain<'a>>,
}

impl Ground<'_> {
    
	pub fn create_pos_on_terrain(&self, x: f32, z: f32) -> Vector3f {
        let y = self.height_at_xz(x, z);
		Vector3f::new(x, y, z)
	}

    pub fn height_at_xz(&self, x: f32, z: f32) -> f32 {
        for terrain_cell in self.terrains.iter() {
            if terrain_cell.is_xz_within_terrain_cell(x, z) {
                let mx = x - terrain_cell.x;
                let mz = z - terrain_cell.z;
                let grid_cell_count = terrain_cell.model.height_map.len();
                let grid_width = Terrain::SIZE / ((grid_cell_count - 1) as f32);
                let grid_x = (mx / grid_width).floor() as usize;
                let grid_z = (mz / grid_width).floor() as usize;
                // clamp due to floating point imprecision?
                let grid_x = cmp::max(0, cmp::min(grid_x, grid_cell_count - 2));
                let grid_z = cmp::max(0, cmp::min(grid_z, grid_cell_count - 2));

                // now find the coords in the rectangle as fraction in [0,1]
                let r_x = (mx % grid_width) / grid_width;
                let r_z = (mz % grid_width) / grid_width;
                // now find which of the two inner triangles we are in
                let (t_a, t_b, t_c) = if r_x + r_z <= 1.0 {
                    let t_a = Vector3f::new(0.0, 0.0, terrain_cell.model.height_map[grid_x][grid_z]);
                    let t_b = Vector3f::new(1.0, 0.0, terrain_cell.model.height_map[grid_x + 1][grid_z]);
                    let t_c = Vector3f::new(0.0, 1.0, terrain_cell.model.height_map[grid_x][grid_z + 1]);
                    (t_a, t_b, t_c)
                } else {
                    let t_a = Vector3f::new(1.0, 0.0, terrain_cell.model.height_map[grid_x + 1][grid_z]);
                    let t_b = Vector3f::new(0.0, 1.0, terrain_cell.model.height_map[grid_x][grid_z + 1]);
                    let t_c = Vector3f::new(1.0, 1.0, terrain_cell.model.height_map[grid_x + 1][grid_z + 1]);
                    (t_a, t_b, t_c)
                };
                let point = Vector2f::new(r_x, r_z);
                let bary_coords = BarycentricCoords::to_barycentric_coords(&point, &t_a, &t_b, &t_c);
                let point_in_3d = BarycentricCoords::from_barycentric_coords(&bary_coords, &t_a, &t_b, &t_c);                             
                return point_in_3d.z;
            }
        }
        return 0.0
    }
}
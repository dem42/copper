use crate::display::{
    Display,
};
use crate::entities::{
    Camera,
};
use crate::math::{
    Matrix3f,
    Matrix4f,
    Vector3f,
    Vector4f,
    distance,  
    f32_min,  
    f32_max,  
};
use std::{    
    f32
};

// the cuboid that we use to find what to draw into the shadow map
// we update the size every frame and we attempt to keep the cuboid as small as possible
// everything in the cuboid will be rendered into the shadow map in the shadow render pass
pub struct ShadowBox {
    farplane_width: f32,
    farplane_height: f32,
    nearplane_width: f32,
    nearplane_height: f32,
    near_plane: f32,
    far_plane: f32,
    pub width: f32,
    pub height: f32,
    pub length: f32,
    pub world_space_center: Vector3f,
    pub obb_corners: [Vector3f; 8],
    pub frustum_corners: [Vector3f; 8],
}

impl ShadowBox {
    pub const OFFSET: f32 = 10.0;
    pub const SHADOW_DISTANCE: f32 = 100.0;

    pub fn new(aspect_ratio: f32, fov_deg: f32, near: f32, far: f32) -> Self {
       let (farplane_width, farplane_height, nearplane_width, nearplane_height) = ShadowBox::compute_frustum_sizes(aspect_ratio, fov_deg, near.abs(), far.abs());

        ShadowBox {
            farplane_width,
            farplane_height,
            nearplane_width,
            nearplane_height,
            near_plane: near,
            far_plane: far,
            width: 0.0,
            height: 0.0,
            length: 0.0,
            world_space_center: Vector3f::zero(),
            obb_corners: Default::default(),
            frustum_corners: Default::default(),
        }
    }

    // does it make sense to transform to light space if all we care about is the world space center
    // and the size of the shadow box (for orthographic projection)
    // a composition of translation and rotation which the transform is a rigid transformation which means it preserves distance between points
    pub fn update(&mut self, camera: &Camera, light_direction_pitch_deg: f32, light_direction_yaw_deg: f32) {        
        let frustum_corners_ws = self.get_frustum_corners_ws(camera);
        self.frustum_corners = frustum_corners_ws.clone();

        self.update_shadow_box_size(frustum_corners_ws, light_direction_pitch_deg, light_direction_yaw_deg);

        // self.obb_corners = ShadowBox::calc_bounding_cuboid_corners_ws(frustum_corners_ws, light_direction_pitch_deg, light_direction_yaw_deg);

        // self.width = distance(&self.obb_corners[0], &self.obb_corners[1]);
        // self.height = distance(&self.obb_corners[0], &self.obb_corners[4]);
        // self.length = distance(&self.obb_corners[0], &self.obb_corners[3]);

        // self.world_space_center = 0.5 * (&self.obb_corners[0] + &self.obb_corners[6]);
    }

    fn compute_frustum_sizes(aspect_ratio: f32, fov_deg: f32, near_dist: f32, far_dist: f32) -> (f32, f32, f32, f32)  {
        let tan_fov_half = (fov_deg / 2.0).to_radians().tan();
        let near_width = 2.0 * near_dist * tan_fov_half;
        let far_width = 2.0 * far_dist * tan_fov_half;
        let near_height = near_width / aspect_ratio;
        let far_height = far_width / aspect_ratio;
        (far_width, far_height, near_width, near_height)
    }

    fn get_frustum_corners_ws(&self, camera: &Camera) -> [Vector3f; 8] {
        let mut corners: [Vector3f; 8] = Default::default();

        // near top right
        corners[0].x = self.nearplane_width / 2.0;
        corners[0].y = self.nearplane_height / 2.0;
        corners[0].z = self.near_plane;
        // near bottom right
        corners[1].x = self.nearplane_width / 2.0;
        corners[1].y = -self.nearplane_height / 2.0;
        corners[1].z = self.near_plane;
        // near bottom left
        corners[2].x = -self.nearplane_width / 2.0;
        corners[2].y = -self.nearplane_height / 2.0;
        corners[2].z = self.near_plane;
        // near top left
        corners[3].x = -self.nearplane_width / 2.0;
        corners[3].y = self.nearplane_height / 2.0;
        corners[3].z = self.near_plane;
        // far top right
        corners[4].x = self.farplane_width / 2.0;
        corners[4].y = self.farplane_height / 2.0;
        corners[4].z = self.far_plane;
        // far bottom right
        corners[5].x = self.farplane_width / 2.0;
        corners[5].y = -self.farplane_height / 2.0;
        corners[5].z = self.far_plane;
        // far bottom left
        corners[6].x = -self.farplane_width / 2.0;
        corners[6].y = -self.farplane_height / 2.0;
        corners[6].z = self.far_plane;
        // far top left
        corners[7].x = -self.farplane_width / 2.0;
        corners[7].y = self.farplane_height / 2.0;
        corners[7].z = self.far_plane;
        
        let ws_to_vs = Matrix4f::create_view_matrix(camera);
        let vs_to_ws = ws_to_vs.inverse();

        let mut temp_v4 = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        for i in 0..corners.len() {
            temp_v4.set_from(&corners[i]);
            temp_v4.w = 1.0; // transform points
            let res = vs_to_ws.transform(&temp_v4);
            corners[i].set_from(&res);
        }
        corners
    }

    fn update_shadow_box_size(&mut self, corners: [Vector3f; 8], light_direction_pitch_deg: f32, light_direction_yaw_deg: f32) {
        let light_orientation = Matrix4f::calculate_rotation_from_rpy(0.0, -light_direction_pitch_deg, -light_direction_yaw_deg);
        let light_orient_inv = light_orientation.transpose();

        // we express every frustum in lightspace
        // in lightspace the cuboid is axis alighned so to figure out the corners of the cuboid we just need to figure out the min,max values
        let mut temp_v4 = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        let mut min_v = Vector3f::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_v = Vector3f::new(f32::MIN, f32::MIN, f32::MIN);
        for i in 0..corners.len() {
            temp_v4.set_from(&corners[i]);
            temp_v4.w = 1.0; // transform points
            let res = light_orientation.transform(&temp_v4);
            min_v.x = f32_min(min_v.x, res.x);
            min_v.y = f32_min(min_v.y, res.y);
            min_v.z = f32_min(min_v.z, res.z);
            max_v.x = f32_max(max_v.x, res.x);
            max_v.y = f32_max(max_v.y, res.y);
            max_v.z = f32_max(max_v.z, res.z);
        }

        self.width = max_v.x - min_v.x;
        self.height = max_v.y - min_v.y;
        self.length = max_v.z - min_v.z;

        let world_space_v4 = Vector4f::new(0.5 * (max_v.x + min_v.x), 0.5 * (max_v.y + min_v.y), 0.5 * (max_v.z + min_v.z), 1.0);
        self.world_space_center = light_orient_inv.transform(&world_space_v4).xyz();
    }

    fn calc_bounding_cuboid_corners_ws(mut corners: [Vector3f; 8], light_direction_pitch_deg: f32, light_direction_yaw_deg: f32) -> [Vector3f; 8] {
        let light_orientation = Matrix4f::calculate_rotation_from_rpy(0.0, light_direction_pitch_deg, light_direction_yaw_deg);
        //let camera_rotation = Matrix4f::identity();
        
        let mut cuboid_faces: [Vector4f; 6] = Default::default();
        for i in 0..3 {            
            cuboid_faces[2*i].x = light_orientation[i][0];
            cuboid_faces[2*i].y = light_orientation[i][1];
            cuboid_faces[2*i].z = light_orientation[i][2];
            cuboid_faces[2*i].w = 0.0;
            cuboid_faces[2*i].normalize();
            cuboid_faces[2*i].w = f32::MAX;

            cuboid_faces[2*i + 1].x = light_orientation[i][0];
            cuboid_faces[2*i + 1].y = light_orientation[i][1];
            cuboid_faces[2*i + 1].z = light_orientation[i][2];
            cuboid_faces[2*i + 1].w = 0.0;
            cuboid_faces[2*i + 1].normalize();
            cuboid_faces[2*i + 1].w = f32::MIN;
        }
        
        // compute the projection of the frustum corners in ws coords onto the cuboid face normals
        // the min projection value should give us one corner point, the max the other corner point
        // we just need to repeat this for all three face normals
        for j in 0..cuboid_faces.len() {
            for i in 0..corners.len() {
                let dprod = cuboid_faces[j].dot_product_v3(&corners[i]);
                if j % 2 == 0 {                    
                    if dprod < cuboid_faces[j].w {
                        cuboid_faces[j].w = dprod;
                    }
                } else {
                    if dprod > cuboid_faces[j].w {
                        cuboid_faces[j].w = dprod;
                    }
                }
            }
        }

        corners[0] = ShadowBox::compute_corner(&cuboid_faces, 0, 2, 5);
        corners[1] = ShadowBox::compute_corner(&cuboid_faces, 1, 2, 5);
        corners[2] = ShadowBox::compute_corner(&cuboid_faces, 1, 2, 4);
        corners[3] = ShadowBox::compute_corner(&cuboid_faces, 0, 2, 4);
        
        corners[4] = ShadowBox::compute_corner(&cuboid_faces, 0, 3, 5);
        corners[5] = ShadowBox::compute_corner(&cuboid_faces, 1, 3, 5);
        corners[6] = ShadowBox::compute_corner(&cuboid_faces, 1, 3, 4);
        corners[7] = ShadowBox::compute_corner(&cuboid_faces, 0, 3, 4);

        corners
    }

    fn compute_corner(cuboid_faces: &[Vector4f; 6], a: usize, b: usize, c: usize) -> Vector3f {
        let mat = Matrix3f::new([[cuboid_faces[a].x, cuboid_faces[a].y, cuboid_faces[a].z], 
            [cuboid_faces[b].x, cuboid_faces[b].y, cuboid_faces[b].z], [cuboid_faces[c].x, cuboid_faces[c].y, cuboid_faces[c].z]]);
        let imat = mat.inverse();
        let d_vec = Vector3f::new(cuboid_faces[a].w, cuboid_faces[b].w, cuboid_faces[c].w);
        let result = imat.transform(&d_vec);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_get_frustume_in_ws_no_rot_trans() {
        let sb = ShadowBox::new(1.0, 60.0, 2.0, 10.0);
        let cam = Camera::new(0.0, 0.0);        
        let corners = sb.get_frustum_corners_ws(&cam);

        let near_corner = 2.0 / f32::sqrt(3.0);
        let far_corner = 10.0 / f32::sqrt(3.0);
        assert_f32_eq!(corners[0].x, near_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].y, near_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].z, 2.0, test_constants::EPS_MEDIUM);
        
        assert_f32_eq!(corners[4].x, far_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[4].y, far_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[4].z, 10.0, test_constants::EPS_MEDIUM);
    }

    #[test]
    fn test_get_frustume_in_ws_with_rot() {
        /*
         * we have a right-hand system so z axis points into screen
         * the rotations are counter-clockwise around the axis (rhs is a counter-clockwise system)
         * so we rotate frustrum from pos z axis axis aligned to neg x axis aligned
         */
        let sb = ShadowBox::new(1.0, 60.0, -2.0, -10.0);
        let mut cam = Camera::new(0.0, 0.0);
        cam.yaw = 90.0;     
        let corners = sb.get_frustum_corners_ws(&cam);

        let near_corner = 2.0 / f32::sqrt(3.0);
        let far_corner = 10.0 / f32::sqrt(3.0);
        assert_f32_eq!(corners[0].x, 2.0, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].y, near_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].z, near_corner, test_constants::EPS_MEDIUM);
        
        assert_f32_eq!(corners[7].x, 10.0, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[7].y, far_corner, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[7].z, -far_corner, test_constants::EPS_MEDIUM);
    }
    
    #[test]
    fn test_get_frustume_in_ws_with_rot_trans() {
        let sb = ShadowBox::new(1.0, 60.0, -2.0, -10.0);
        let mut cam = Camera::new(0.0, 0.0);
        cam.yaw = 90.0;
        cam.position.x = 0.5;
        cam.position.y = 0.5;
        cam.position.z = 0.5;
        let corners = sb.get_frustum_corners_ws(&cam);

        let near_corner = 2.0 / f32::sqrt(3.0);
        let far_corner = 10.0 / f32::sqrt(3.0);
        assert_f32_eq!(corners[0].x, 2.0 + 0.5, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].y, near_corner + 0.5, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[0].z, near_corner + 0.5, test_constants::EPS_MEDIUM);
        
        assert_f32_eq!(corners[7].x, 10.0 + 0.5, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[7].y, far_corner + 0.5, test_constants::EPS_MEDIUM);
        assert_f32_eq!(corners[7].z, -far_corner + 0.5, test_constants::EPS_MEDIUM);
    }

    #[test]
    fn test_shadow_cuboid_plane_normals() {
        let sb = ShadowBox::new(1.0, 60.0, -2.0, -10.0);
        let mut cam = Camera::new(0.0, 0.0);
        cam.yaw = 90.0;
        cam.position.x = 0.5;
        cam.position.y = 0.5;
        cam.position.z = 0.5;
        let corners = sb.get_frustum_corners_ws(&cam);
        let obb_corners = ShadowBox::calc_bounding_cuboid_corners_ws(corners, -90.0, 0.0);

        let far_corner = 10.0 / f32::sqrt(3.0);
        let obb_min = Vector3f::new(2.0 + 0.5, -far_corner + 0.5, -far_corner + 0.5);
        let obb_max = Vector3f::new(10.0 + 0.5, far_corner + 0.5, far_corner + 0.5);
        
        assert_f32_eq!(obb_corners[0].x, obb_min.x, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[0].y, obb_min.y, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[0].z, obb_min.z, test_constants::EPS_MEDIUM);

        assert_f32_eq!(obb_corners[1].x, obb_max.x, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[1].y, obb_min.y, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[1].z, obb_min.z, test_constants::EPS_MEDIUM);

        assert_f32_eq!(obb_corners[3].x, obb_min.x, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[3].y, obb_max.y, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[3].z, obb_min.z, test_constants::EPS_MEDIUM);

        assert_f32_eq!(obb_corners[4].x, obb_min.x, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[4].y, obb_min.y, test_constants::EPS_MEDIUM);
        assert_f32_eq!(obb_corners[4].z, obb_max.z, test_constants::EPS_MEDIUM);
    }

    #[test]
    fn test_shadow_box_update() {
        let mut sb = ShadowBox::new(1.0, 60.0, -2.0, -10.0);
        let mut cam = Camera::new(0.0, 0.0);
        cam.yaw = 90.0;
        cam.position.x = 0.5;
        cam.position.y = 0.5;
        cam.position.z = 0.5;
        
        let far_corner = 10.0 / f32::sqrt(3.0);
        let obb_min = Vector3f::new(2.0 + 0.5, -far_corner + 0.5, -far_corner + 0.5);
        let obb_max = Vector3f::new(10.0 + 0.5, far_corner + 0.5, far_corner + 0.5);

        sb.update(&cam, -90.0, 0.0);
        
        assert_f32_eq!(sb.width, (obb_max.x - obb_min.x), test_constants::EPS_PRECISE);
        assert_f32_eq!(sb.height, (obb_max.y - obb_min.y), test_constants::EPS_PRECISE);
        assert_f32_eq!(sb.length, (obb_max.z - obb_min.z), test_constants::EPS_PRECISE);

        assert_f32_eq!(sb.world_space_center.x, 0.5 * (obb_max.x + obb_min.x), test_constants::EPS_PRECISE);
        assert_f32_eq!(sb.world_space_center.y, 0.5 * (obb_max.y + obb_min.y), test_constants::EPS_PRECISE);        
        assert_f32_eq!(sb.world_space_center.z, 0.5 * (obb_max.z + obb_min.z), absolute=test_constants::EPS_PRECISE, relative=test_constants::EPS_BAD);
    }
}
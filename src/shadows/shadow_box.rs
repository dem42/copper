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
    pub width: f32,
    pub height: f32,
    pub length: f32,
    pub world_space_center: Vector3f,
}

impl ShadowBox {
    const OFFSET: f32 = 10.0;
    const SHADOW_DISTANCE: f32 = 100.0;

    pub fn new(aspect_ratio: f32) -> Self {
       let (farplane_width, farplane_height, nearplane_width, nearplane_height) = ShadowBox::compute_frustum_sizes(aspect_ratio);

        ShadowBox {
            farplane_width,
            farplane_height,
            nearplane_width,
            nearplane_height,
            width: 0.0,
            height: 0.0,
            length: 0.0,
            world_space_center: Vector3f::zero(),
        }
    }

    // does it make sense to transform to light space if all we care about is the world space center
    // and the size of the shadow box (for orthographic projection)
    // a composition of translation and rotation which the transform is a rigid transformation which means it preserves distance between points
    pub fn update(&mut self, camera: &Camera) {        
        let frustum_corners_ws = self.calc_camera_frustum_corners_in_worldspace(camera, Display::NEAR, ShadowBox::SHADOW_DISTANCE);

        self.width = distance(&frustum_corners_ws[0], &frustum_corners_ws[1]);
        self.height = distance(&frustum_corners_ws[0], &frustum_corners_ws[4]);
        self.length = distance(&frustum_corners_ws[0], &frustum_corners_ws[3]);

        self.world_space_center.x = 0.5 * (frustum_corners_ws[0].x + frustum_corners_ws[1].x);
        self.world_space_center.y = 0.5 * (frustum_corners_ws[0].y + frustum_corners_ws[4].y);
        self.world_space_center.z = 0.5 * (frustum_corners_ws[0].z + frustum_corners_ws[3].z);
    }

    fn compute_frustum_sizes(aspect_ratio: f32) -> (f32, f32, f32, f32)  {
        let tan_fov_half = (Display::FOV_HORIZONTAL / 2.0).to_radians().tan();
        let near_width = -2.0 * Display::NEAR * tan_fov_half;
        let far_width = -2.0 * Display::FAR * tan_fov_half;
        let near_height = near_width / aspect_ratio;
        let far_height = far_width / aspect_ratio;
        (far_width, far_height, near_width, near_height)
    }

    fn calc_camera_frustum_corners_in_worldspace(&self, camera: &Camera, near: f32, far: f32) -> [Vector3f; 8] {
        let camera_rotation = Matrix4f::calculate_rotation_from_rpy(camera.roll, camera.pitch, camera.yaw);
        let mut corners: [Vector3f; 8] = Default::default();

        // near top right
        corners[0].x = self.nearplane_width / 2.0;
        corners[0].y = self.nearplane_height / 2.0;
        corners[0].z = near;
        // near bottom right
        corners[1].x = self.nearplane_width / 2.0;
        corners[1].y = -self.nearplane_height / 2.0;
        corners[1].z = near;
        // near bottom left
        corners[2].x = -self.nearplane_width / 2.0;
        corners[2].y = -self.nearplane_height / 2.0;
        corners[2].z = near;
        // near top left
        corners[3].x = -self.nearplane_width / 2.0;
        corners[3].y = self.nearplane_height / 2.0;
        corners[3].z = near;
        // far top left
        corners[4].x = -self.farplane_width / 2.0;
        corners[4].y = self.farplane_height / 2.0;
        corners[4].z = far;
        // far top right
        corners[5].x = self.farplane_width / 2.0;
        corners[5].y = self.farplane_height / 2.0;
        corners[5].z = far;
        // far bottom right
        corners[6].x = self.farplane_width / 2.0;
        corners[6].y = -self.farplane_height / 2.0;
        corners[6].z = far;
        // far bottom left
        corners[7].x = -self.farplane_width / 2.0;
        corners[7].y = -self.farplane_height / 2.0;
        corners[7].z = far;

        for i in 0..corners.len() {
            corners[i] += &camera.position;
        }

        let mut cuboid_faces: [Vector4f; 6] = Default::default();
        for i in 0..3 {            
            cuboid_faces[2*i].x = camera_rotation[i][0];
            cuboid_faces[2*i].y = camera_rotation[i][1];
            cuboid_faces[2*i].z = camera_rotation[i][2];
            cuboid_faces[2*i].w = 0.0;
            cuboid_faces[2*i].normalize();
            cuboid_faces[2*i].w = f32::MAX;

            cuboid_faces[2*i + 1].x = camera_rotation[i][0];
            cuboid_faces[2*i + 1].y = camera_rotation[i][1];
            cuboid_faces[2*i + 1].z = camera_rotation[i][2];
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

    #[test]
    fn test_get_frustume_in_ws() {
     
    }

    #[test]
    fn test_shadow_cuboid_plane_normals() {

    }
}
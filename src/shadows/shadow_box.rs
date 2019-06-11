use crate::display::{
    Display,
};
use crate::entities::{
    Camera,
};
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,    
};
use std::cmp;

// the cuboid that we use to find what to draw into the shadow map
// we update the size every frame and we attempt to keep the cuboid as small as possible
// everything in the cuboid will be rendered into the shadow map in the shadow render pass
pub struct ShadowBox {
    farplane_width: f32,
    farplane_height: f32,
    nearplane_width: f32,
    nearplane_height: f32,
    world_to_light_transform: Matrix4f,
    frustum_min_corner: Vector3f,
    frustum_max_corner: Vector3f,
}

impl ShadowBox {
    const OFFSET: f32 = 10.0;    
    const UP: Vector4f = Vector4f {x: 0.0, y: 1.0, z: 0.0, w: 0.0};
    const FORWARD: Vector4f = Vector4f {x: 0.0, y: 0.0, z: -1.0, w: 0.0};
    const SHADOW_DISTANCE: f32 = 100.0;

    pub fn new(world_to_light_transform: Matrix4f, aspect_ratio: f32) -> Self {

       let (farplane_width, farplane_height, nearplane_width, nearplane_height) = ShadowBox::compute_frustum_sizes(aspect_ratio);

        ShadowBox {
            farplane_width,
            farplane_height,
            nearplane_width,
            nearplane_height, 
            world_to_light_transform,
            frustum_min_corner: Vector3f::zero(),
            frustum_max_corner: Vector3f::zero(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        let camera_rotation = Matrix4f::calculate_rotation_from_rpy(camera.roll, camera.pitch, camera.yaw);
        let forward_view_space = camera_rotation.transform(&ShadowBox::FORWARD).xyz();
        let frustum_near_center = &forward_view_space * (-Display::NEAR); 
        let frustum_far_center = &forward_view_space * ShadowBox::SHADOW_DISTANCE;

        let camera_frustum_corners_in_lightspace = self.calc_camera_frustum_corners_in_lightspace(camera_rotation, forward_view_space, frustum_near_center, frustum_far_center);

        self.frustum_min_corner.x = camera_frustum_corners_in_lightspace[0].x;
        self.frustum_min_corner.y = camera_frustum_corners_in_lightspace[0].y;
        self.frustum_min_corner.z = camera_frustum_corners_in_lightspace[0].z;
        self.frustum_max_corner.x = camera_frustum_corners_in_lightspace[0].x;
        self.frustum_max_corner.y = camera_frustum_corners_in_lightspace[0].y;
        self.frustum_max_corner.z = camera_frustum_corners_in_lightspace[0].z;

        for corner in camera_frustum_corners_in_lightspace.into_iter() {
            if self.frustum_min_corner.x > corner.x {
                self.frustum_min_corner.x = corner.x;
            } else if self.frustum_max_corner.x < corner.x {
                self.frustum_max_corner.x = corner.x;
            }

            if self.frustum_min_corner.y > corner.y {
                self.frustum_min_corner.y = corner.y;
            } else if self.frustum_max_corner.y < corner.y {
                self.frustum_max_corner.y = corner.y;
            }

            if self.frustum_min_corner.z > corner.z {
                self.frustum_min_corner.z = corner.z;
            } else if self.frustum_max_corner.z < corner.z {
                self.frustum_max_corner.z = corner.z;
            }
        }
    }

    fn compute_frustum_sizes(aspect_ratio: f32) -> (f32, f32, f32, f32)  {
        let tan_fov_half = (Display::FOV_HORIZONTAL / 2.0).to_radians().tan();
        let near_width = -2.0 * Display::NEAR * tan_fov_half;
        let far_width = -2.0 * Display::FAR * tan_fov_half;
        let near_height = near_width / aspect_ratio;
        let far_height = far_width / aspect_ratio;
        (far_width, far_height, near_width, near_height)
    }

    fn calc_camera_frustum_corners_in_lightspace(&self, camera_rotation: Matrix4f, fwd_view_space: Vector3f, center_near: Vector3f, center_far: Vector3f) -> [Vector3f; 8] {        
        let mut corners: [Vector3f; 8] = Default::default();

        // near top right
        corners[0].x = self.nearplane_width / 2.0;
        corners[0].y = self.nearplane_height / 2.0;
        corners[0].z = Display::NEAR;
        // near bottom right
        corners[1].x = self.nearplane_width / 2.0;
        corners[1].y = -self.nearplane_height / 2.0;
        corners[1].z = Display::NEAR;
        // near bottom left
        corners[2].x = -self.nearplane_width / 2.0;
        corners[2].y = -self.nearplane_height / 2.0;
        corners[2].z = Display::NEAR;
        // near top left
        corners[3].x = -self.nearplane_width / 2.0;
        corners[3].y = self.nearplane_height / 2.0;
        corners[3].z = Display::NEAR;
        // far top left
        corners[4].x = -self.farplane_width / 2.0;
        corners[4].y = self.farplane_height / 2.0;
        corners[4].z = Display::FAR;
        // far top right
        corners[5].x = self.farplane_width / 2.0;
        corners[5].y = self.farplane_height / 2.0;
        corners[5].z = Display::FAR;
        // far bottom right
        corners[6].x = self.farplane_width / 2.0;
        corners[6].y = -self.farplane_height / 2.0;
        corners[6].z = Display::FAR;
        // far bottom left
        corners[7].x = -self.farplane_width / 2.0;
        corners[7].y = -self.farplane_height / 2.0;
        corners[7].z = Display::FAR;

        for i in 0..corners.len() {
            self.transform_vertex_to_lightspace(&mut corners[i]);
        }

        corners
    }

    fn transform_vertex_to_lightspace(&self, vertex: &mut Vector3f) {

    }
}
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
    const DOWN: Vector4f = Vector4f {x: 0.0, y: 0.0, z: -1.0, w: 0.0};
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
        let camera_frustum_corners_in_lightspace = ShadowBox::calc_camera_frustum_corners_in_lightspace(camera_rotation);

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

    fn calc_camera_frustum_corners_in_lightspace(camera_rotation: Matrix4f) -> [Vector3f; 8] {
        unimplemented!()
    }
}
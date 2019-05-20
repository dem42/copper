use crate::display::{
    Display,
};
use crate::math::{
    Vector4f,
};

// the cuboid that we use to find what to draw into the shadow map
// we update the size every frame and we attempt to keep the cuboid as small as possible
// everything in the cuboid will be rendered into the shadow map in the shadow render pass
pub struct ShadowBox {
    farplane_width: f32,
    farplane_height: f32,
    nearplane_width: f32,
    nearplane_height: f32,
}

impl ShadowBox {
    const OFFSET: f32 = 10.0;
    const UP: Vector4f = Vector4f {x: 0.0, y: 1.0, z: 0.0, w: 0.0};
    const DOWN: Vector4f = Vector4f {x: 0.0, y: 0.0, z: -1.0, w: 0.0};
    const SHADOW_DISTANCE: f32 = 100.0;

    pub fn new(aspect_ratio: f32) -> Self {

       let (farplane_width, farplane_height, nearplane_width, nearplane_height) = ShadowBox::compute_frustum_sizes(aspect_ratio);

        ShadowBox {
            farplane_width,
            farplane_height,
            nearplane_width,
            nearplane_height, 
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
}
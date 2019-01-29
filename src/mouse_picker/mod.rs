use crate::math::{
    Matrix4f,
    Vector2f,
    Vector3f,
    Vector4f,
};
use crate::entities::{
    Camera,
    Ground,
};
use crate::display::{
    Display,
    Keyboard,
};

pub struct MousePicker;

impl MousePicker {
    pub fn new() -> MousePicker {
        MousePicker {}
    }

    pub fn update(&mut self, display: &Display, proj_matrix: &Matrix4f, camera: &Camera, ground: &Ground) -> Option<Vector3f> {
        if display.is_mouse_select_active() {
            let mouse_ray = self.calculate_mouse_ray(display, proj_matrix, camera);
            MousePicker::search_ground_intersection(mouse_ray, camera, ground)
        } else {
            None
        }        
        //println!("Mouse ray is: {:?}. Mouse is at ({},{})", mouse_ray, display.mouse_pos.cur_x, display.mouse_pos.cur_y);
    }

    fn search_ground_intersection(ray: Vector3f, camera: &Camera, ground: &Ground) -> Option<Vector3f> {

        const IT_LIMIT: u8 = 100;
        let mut low = 0.0;
        let mut high = 50_000.0;
        let mut mid = 0.0;
        let mut it_cnt = 0;
        let mut was_below = false;
        let mut was_above = false;
        // bin search implementation doesnt let you look up from under terrain (has to do with how low and high is set)
        // and it may give wrong result if terrain is bumpy and first jump takes you too far
        while it_cnt < IT_LIMIT {
            mid = (low + high) / 2.0;
            let ray_x = ray.x * mid + camera.position.x;
            let ray_y = ray.y * mid + camera.position.y;
            let ray_z = ray.z * mid + camera.position.z;
            let terrain_height = ground.height_at_xz(ray_x, ray_z);
            if terrain_height < ray_y {
                low = mid;                
                was_above = true;
            } else {                
                high = mid;
                was_below = true;
            }
            it_cnt += 1;
        }

        if was_above && was_below {
            Some(Vector3f::new(ray.x * mid, ray.y * mid, ray.z * mid))
        } else {
            None
        }
    }

    fn calculate_mouse_ray(&self, display: &Display, proj_matrix: &Matrix4f, camera: &Camera) -> Vector3f {
        let mouse_x = display.mouse_pos.cur_x as f32;
        let mouse_y = display.mouse_pos.cur_y as f32;
        let ndc_coords = MousePicker::viewport_to_normalized_device_coords(mouse_x, mouse_y, display);
        // not needed to reverse perspective projection in this case since we want to find ray which doesnt have a z
        let clip_coords = Vector4f::new(ndc_coords.x, ndc_coords.y, -1.0, 1.0);
        let eye_coords = MousePicker::to_eye_coords(&clip_coords, proj_matrix);
        let world_coords = MousePicker::to_world_coords(&eye_coords, &Matrix4f::create_view_matrix(camera));
        world_coords
    }

    fn to_world_coords(eye_coords: &Vector4f, view_matrix: &Matrix4f) -> Vector3f {
        let inverse_proj_mat = view_matrix.inverse();
        let result4 = inverse_proj_mat.transform(eye_coords);
        let mut result = Vector3f::new(result4.x, result4.y, result4.z);
        result.normalize();
        result
    }

    fn to_eye_coords(clip_coords: &Vector4f, proj_matrix: &Matrix4f) -> Vector4f {
        let inverse_proj_mat = proj_matrix.inverse();
        let mut result = inverse_proj_mat.transform(clip_coords);
        // check whether this next step is necessary .. we are interested in ray into scene so we drop z and w
        result.z = -1.0;
        result.w = 0.0;
        result
    }

    fn viewport_to_normalized_device_coords(mouse_x: f32, mouse_y: f32, display: &Display) -> Vector2f {
        // from [(0,height), (width, 0)] to [(-1,-1), (1,1)] 
        // move by half w/h to center then scale by half w/h down
        let (w, h) = display.get_size();
        Vector2f::new((2.0*mouse_x - w)/w, (h - 2.0*mouse_y)/h)
    }
}


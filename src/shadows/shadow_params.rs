#[derive(Debug, Clone)]
pub struct ShadowParams {
    pub shadow_map_texture: u32,
    pub shadow_distance: f32,
    pub shadow_map_size: usize,    
}
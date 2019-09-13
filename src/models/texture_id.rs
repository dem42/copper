#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureId {
    Empty,
    Loading(u32),
    Loaded(u32),
    FboTexture(u32),    
}

impl TextureId {
    pub fn unwrap(&self) -> u32 {
        match self {
            TextureId::Empty => panic!("Attempted to access a texture we never attempted to load"),
            TextureId::Loading(_) => panic!("Attempted to access a texture that hasn't been loaded"),
            TextureId::Loaded(id) => *id,
            TextureId::FboTexture(id) => *id,
        }
    }
}

impl Default for TextureId {
    fn default() -> Self {
        TextureId::Empty
    }
}
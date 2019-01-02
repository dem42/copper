use super::loader::{
    ModelLoader,
    TexturedModel,
    TerrainTexture,  
    TerrainTexturePack,
    RawModel,
    TextureFlags,
    TerrainModel,
};
use crate::entities::Terrain;
use crate::obj_converter::load_obj_model;
use std::collections::HashMap;

#[derive(Default)]
pub struct ResourceManager {
    loader: ModelLoader,
    texture_pack: Option<TerrainTexturePack>,
    blend_texture: Option<TerrainTexture>,
    terrain_model: Option<TerrainModel>,

    models: HashMap<ModelType, TexturedModel>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ModelType {
    Grass,
    Fern,
    Player,
    Tree,
    LowPolyTree,
    Flowers,
    Crate,
}
#[derive(Default)]
pub struct ModelProps {
    pub has_transparency: bool,
    pub uses_fake_lighting: bool,
    pub uses_mipmaps: bool,
}
impl ModelProps {
    fn get_texture_flags(&self) -> u8 {
        let mut res = 0;
        if self.uses_mipmaps {
            res |= TextureFlags::MIPMAP as u8;
        }
        res
    }
}

pub struct Model(ModelType, &'static str, &'static str, &'static ModelProps);

pub struct Models;

impl Models {
    pub const COMMON_PROPS: ModelProps = ModelProps{has_transparency: false, uses_fake_lighting: false, uses_mipmaps: true};
    //pub const NO_MIPMAP_PROPS: ModelProps = ModelProps{has_transparency: false, uses_fake_lighting: false, uses_mipmaps: false};
    pub const TRANSPARENCY_PROPS: ModelProps = ModelProps{has_transparency: false, uses_fake_lighting: false, uses_mipmaps: true};
    pub const GRASS_PROPS: ModelProps = ModelProps{has_transparency: false, uses_fake_lighting: false, uses_mipmaps: true};
    
    pub const PLAYER: Model = Model(ModelType::Player, "res/models/person.obj", "res/textures/playerTexture.png", &Models::COMMON_PROPS);
    pub const TREE: Model = Model(ModelType::Tree, "res/models/tree.obj", "res/textures/tree.png", &Models::COMMON_PROPS);
    pub const LOW_POLY_TREE: Model = Model(ModelType::LowPolyTree, "res/models/lowPolyTree.obj", "res/textures/lowPolyTree.png", &Models::COMMON_PROPS);
    pub const FERN: Model = Model(ModelType::Fern, "res/models/fern.obj", "res/textures/fern.png", &Models::TRANSPARENCY_PROPS);
    pub const GRASS: Model = Model(ModelType::Grass, "res/models/grassModel.obj", "res/textures/grassTexture.png", &Models::GRASS_PROPS);
    pub const FLOWERS: Model = Model(ModelType::Flowers, "res/models/grassModel.obj", "res/textures/flower.png", &Models::GRASS_PROPS);
    pub const CRATE: Model = Model(ModelType::Crate, "res/models/box.obj", "res/textures/box.png", &Models::COMMON_PROPS);
}


impl ResourceManager {
    
    pub fn init(&mut self, Model(model_type, obj_file, texture_file, model_props): &Model) {
        // thread safe coz only one mutable reference to resource manager can be held
        if self.models.contains_key(model_type) {
            return;
        }
        let model_data = load_obj_model(obj_file).expect(&format!("Unable to load {}", obj_file));
        let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
        let mut texture = self.loader.load_texture(texture_file, model_props.get_texture_flags());
        texture.has_transparency = model_props.has_transparency;
        texture.uses_fake_lighting = model_props.uses_fake_lighting;
        let model = TexturedModel { raw_model, texture };

        self.models.insert(model_type.clone(), model);
    }

    pub fn model(&self, model_type: ModelType) -> &TexturedModel {
        self.models.get(&model_type).expect(&format!("Need to call init_model({:?}) before accessing the model", model_type))
    }
    
    pub fn init_terrain_textures(&mut self) {
        let texture_flags = TextureFlags::MIPMAP as u8;
        if let None = self.texture_pack {
            let background_texture = self.loader.load_terrain_texture("res/textures/terrain/grassy2.png", texture_flags);
            let r_texture = self.loader.load_terrain_texture("res/textures/terrain/mud.png", texture_flags);
            let g_texture = self.loader.load_terrain_texture("res/textures/terrain/grassFlowers.png", texture_flags);
            let b_texture = self.loader.load_terrain_texture("res/textures/terrain/path.png", texture_flags);
            self.texture_pack = Some(TerrainTexturePack { background_texture, r_texture, g_texture, b_texture, });
        }
        if let None = self.blend_texture {
            self.blend_texture = Some(self.loader.load_terrain_texture("res/textures/terrain/blendMap.png", texture_flags));
        }
    }

    pub fn terrain_pack(&self) -> &TerrainTexturePack {
        self.texture_pack.as_ref().expect("Need to call init_terrain_textures before accessing the textures")
    }

    pub fn blend_texture(&self) -> &TerrainTexture {
        self.blend_texture.as_ref().expect("Need to call init_terrain_textures before accessing the textures")
    }

    pub fn init_terrain_model(&mut self) {
        if let None = self.terrain_model {
            let model = Terrain::generate_terrain(&mut self.loader, "res/textures/terrain/heightmap.png");
            self.terrain_model = Some(model);
        }
    }

    pub fn terrain_model(&self) -> &TerrainModel {
        self.terrain_model.as_ref().expect("Need to call init_terrain_model before accessing the model")
    }
}
use super::loader::{
    ModelLoader,
    TexturedModel,
    TerrainTexture,  
    TerrainTexturePack,
    TextureFlags,
    TerrainModel,
    GuiModel,
    SkyboxModel,
    WaterModel,
};
use crate::entities::Terrain;
use crate::obj_converter::{
    load_obj_model,
    load_simple_obj_model
};
use std::collections::HashMap;

#[derive(Default)]
pub struct ResourceManager {
    loader: ModelLoader,
    texture_pack: Option<TerrainTexturePack>,
    blend_texture: Option<TerrainTexture>,
    terrain_model: Option<TerrainModel>,
    gui_model: Option<GuiModel>,
    skybox_model: Option<SkyboxModel>,
    water_model: Option<WaterModel>,

    models: HashMap<ModelType, TexturedModel>,
    gui_textures: HashMap<&'static str, u32>,
}

pub enum ResType {
    WaterModel,
    SkyboxModel,
    GuiModel,
    GuiRes(&'static str),
    TexAndModel {tex: &'static str, model: &'static str, model_props: ModelProps},
    TerrainTexPack {blend: &'static str, back: &'static str, a_chan: &'static str, g_chan: &'static str, b_chan: &'static str},
    TerrainModel {heightmap: &'static str},
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
    Lamp,
    ToonRocks,
    BobbleTree,
    Barrel,
}

pub struct AtlasProps(usize);

pub struct ModelProps {
    pub has_transparency: bool,
    pub uses_fake_lighting: bool,
    pub uses_mipmaps: bool,
    pub shine_damper: f32,
    pub reflectivity: f32,
    pub atlas_props: AtlasProps,
    pub normal_map: Option<&'static str>,
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
    const GUI_PROPS: ModelProps = ModelProps {
        has_transparency: false, 
        uses_fake_lighting: false, 
        uses_mipmaps: false,
        shine_damper: 1.0,
        reflectivity: 0.0, 
        atlas_props: AtlasProps(1),
        normal_map: None,
    };
    const COMMON_PROPS: ModelProps = ModelProps {
        has_transparency: false, 
        uses_fake_lighting: false, 
        uses_mipmaps: true,
        shine_damper: 1.0,
        reflectivity: 0.0,  
        atlas_props: AtlasProps(1),
        normal_map: None,
    };
    const SHINY_PROPS: ModelProps = ModelProps {
        has_transparency: false, 
        uses_fake_lighting: false, 
        uses_mipmaps: true,
        shine_damper: 20.0,
        reflectivity: 0.6,  
        atlas_props: AtlasProps(1),
        normal_map: None,
    };
    const FERN_PROPS: ModelProps = ModelProps { 
        has_transparency: true, 
        uses_fake_lighting: false, 
        uses_mipmaps: true, 
        shine_damper: 1.0,
        reflectivity: 0.0, 
        atlas_props: AtlasProps(2),
        normal_map: None,
    };    
    const GRASS_PROPS: ModelProps = ModelProps { 
        has_transparency: true, 
        uses_fake_lighting: true, 
        uses_mipmaps: true, 
        shine_damper: 1.0,
        reflectivity: 0.0, 
        atlas_props: AtlasProps(1),
        normal_map: None,
    };
    // point light is inside the lamp. to get it to light up the outer faces we make the outer faces have a vector that points up
    const LAMP_PROPS: ModelProps = ModelProps { 
        has_transparency: false, 
        uses_fake_lighting: true, 
        uses_mipmaps: true, 
        shine_damper: 1.0,
        reflectivity: 0.0, 
        atlas_props: AtlasProps(1),
        normal_map: None,
    };
    const BARREL_PROPS: ModelProps = ModelProps { 
        has_transparency: false, 
        uses_fake_lighting: false, 
        uses_mipmaps: true, 
        shine_damper: 10.0,
        reflectivity: 0.5, 
        atlas_props: AtlasProps(1),
        normal_map: Some("res/textures/normal_maps/barrelNormal.png"),
    };    
    
    pub const PLAYER: Model = Model(ModelType::Player, "res/models/person.obj", "res/textures/playerTexture.png", &Models::COMMON_PROPS);
    pub const TREE: Model = Model(ModelType::Tree, "res/models/tree.obj", "res/textures/tree.png", &Models::COMMON_PROPS);
    pub const LOW_POLY_TREE: Model = Model(ModelType::LowPolyTree, "res/models/lowPolyTree.obj", "res/textures/lowPolyTree.png", &Models::COMMON_PROPS);
    pub const FERN: Model = Model(ModelType::Fern, "res/models/fern.obj", "res/textures/atlases/fern.png", &Models::FERN_PROPS);
    pub const GRASS: Model = Model(ModelType::Grass, "res/models/grassModel.obj", "res/textures/grassTexture.png", &Models::GRASS_PROPS);
    pub const FLOWERS: Model = Model(ModelType::Flowers, "res/models/grassModel.obj", "res/textures/flower.png", &Models::GRASS_PROPS);
    pub const CRATE: Model = Model(ModelType::Crate, "res/models/box.obj", "res/textures/box.png", &Models::COMMON_PROPS);
    pub const LAMP: Model = Model(ModelType::Lamp, "res/models/lamp.obj", "res/textures/lamp.png", &Models::LAMP_PROPS);
    pub const TOON_ROCKS: Model = Model(ModelType::ToonRocks, "res/models/toonRocks.obj", "res/textures/toonRocks.png", &Models::SHINY_PROPS);
    pub const BOBBLE_TREE: Model = Model(ModelType::BobbleTree, "res/models/bobbleTree.obj", "res/textures/bobbleTree.png", &Models::COMMON_PROPS);
    pub const BARREL: Model = Model(ModelType::Barrel, "res/models/barrel.obj", "res/textures/barrel.png", &Models::BARREL_PROPS);
}


impl ResourceManager {

    pub const HEALTHBAR_TEXTURE: &'static str = "res/textures/health.png";
    pub const GUI_BACKGROUND_TEXTURE: &'static str = "res/textures/gui_background.png";
    
    pub fn init(&mut self, Model(model_type, obj_file, texture_file, model_props): &Model) {
        // thread safe coz only one mutable reference to resource manager can be held
        if self.models.contains_key(model_type) {
            return;
        }
        
        let raw_model = if let Some(_normal_map_texture) = model_props.normal_map {
            let model_data = load_obj_model(obj_file, true).expect(&format!("Unable to load {}", obj_file));
            self.loader.load_to_vao_with_normal_map(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals, &model_data.tangents)
        } else {
            let model_data = load_simple_obj_model(obj_file).expect(&format!("Unable to load simple {}", obj_file));
            self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals)
        }; 
        
        let mut texture = self.loader.load_texture(texture_file, model_props.get_texture_flags());
        texture.has_transparency = model_props.has_transparency;
        texture.uses_fake_lighting = model_props.uses_fake_lighting;
        texture.shine_damper = model_props.shine_damper;
        texture.reflectivity = model_props.reflectivity;
        texture.number_of_rows_in_atlas = model_props.atlas_props.0;
        let model = TexturedModel { raw_model, texture };

        self.models.insert(model_type.clone(), model);
    }

    pub fn model(&self, model_type: ModelType) -> TexturedModel {
        self.models.get(&model_type).expect(&format!("Need to call init_model({:?}) before accessing the model", model_type)).clone()
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

    pub fn terrain_pack(&self) -> TerrainTexturePack {
        self.texture_pack.clone().expect("Need to call init_terrain_textures before accessing the textures")
    }

    pub fn blend_texture(&self) -> TerrainTexture {
        self.blend_texture.clone().expect("Need to call init_terrain_textures before accessing the textures")
    }

    pub fn init_terrain_model(&mut self) {
        if let None = self.terrain_model {
            let model = Terrain::generate_terrain(&mut self.loader, "res/textures/terrain/heightmap.png");
            self.terrain_model = Some(model);
        }
    }

    pub fn terrain_model(&self) -> TerrainModel {
        self.terrain_model.clone().expect("Need to call init_terrain_model before accessing the model")
    }

    pub fn init_gui_textures(&mut self) {        
        let props = Models::GUI_PROPS;
        if !self.gui_textures.contains_key(ResourceManager::HEALTHBAR_TEXTURE) {
            let texture_id = self.loader.load_gui_texture(ResourceManager::HEALTHBAR_TEXTURE, props.get_texture_flags());
            self.gui_textures.insert(ResourceManager::HEALTHBAR_TEXTURE, texture_id);
        }

        if !self.gui_textures.contains_key(ResourceManager::GUI_BACKGROUND_TEXTURE) {
            let texture_id = self.loader.load_gui_texture(ResourceManager::GUI_BACKGROUND_TEXTURE, props.get_texture_flags());
            self.gui_textures.insert(ResourceManager::GUI_BACKGROUND_TEXTURE, texture_id);
        }
    }

    pub fn get_gui_texture(&self, texture_name: &str) -> u32 {
         let tex_id = self.gui_textures.get(texture_name).expect("Must call init_gui_textures first");
         *tex_id
    }

    pub fn init_gui_model(&mut self) {
        if let None = self.gui_model {
            // create quad that covers full screen -> we will scale it to create guis
            let positions = vec!{
                -1.0, 1.0,
                -1.0, -1.0,
                1.0, 1.0,
                1.0, -1.0,
            };
            let raw_model = self.loader.load_simple_model_to_vao(&positions, 2);
            self.gui_model = Some(GuiModel {
                raw_model,
            });
        }
    }

    pub fn gui_model(&self) -> GuiModel {
        self.gui_model.clone().expect("Need to call init_gui_model before accessing gui model")
    }

    pub fn skybox(&self) -> SkyboxModel {
        self.skybox_model.clone().expect("Need to call init_skybox first")
    }

    pub fn init_skybox(&mut self) {
        if let None = self.skybox_model {
            let day_texture_id = self.loader.load_cube_map("res/textures/cube_maps/day_skybox");
            let night_texture_id = self.loader.load_cube_map("res/textures/cube_maps/night_skybox");
            
            const SIZE: f32 = 500.0;
            let positions = vec![
                -SIZE,  SIZE, -SIZE,
                -SIZE, -SIZE, -SIZE,
                SIZE, -SIZE, -SIZE,
                SIZE, -SIZE, -SIZE,
                SIZE,  SIZE, -SIZE,
                -SIZE,  SIZE, -SIZE,

                -SIZE, -SIZE,  SIZE,
                -SIZE, -SIZE, -SIZE,
                -SIZE,  SIZE, -SIZE,
                -SIZE,  SIZE, -SIZE,
                -SIZE,  SIZE,  SIZE,
                -SIZE, -SIZE,  SIZE,

                SIZE, -SIZE, -SIZE,
                SIZE, -SIZE,  SIZE,
                SIZE,  SIZE,  SIZE,
                SIZE,  SIZE,  SIZE,
                SIZE,  SIZE, -SIZE,
                SIZE, -SIZE, -SIZE,

                -SIZE, -SIZE,  SIZE,
                -SIZE,  SIZE,  SIZE,
                SIZE,  SIZE,  SIZE,
                SIZE,  SIZE,  SIZE,
                SIZE, -SIZE,  SIZE,
                -SIZE, -SIZE,  SIZE,

                -SIZE,  SIZE, -SIZE,
                SIZE,  SIZE, -SIZE,
                SIZE,  SIZE,  SIZE,
                SIZE,  SIZE,  SIZE,
                -SIZE,  SIZE,  SIZE,
                -SIZE,  SIZE, -SIZE,

                -SIZE, -SIZE, -SIZE,
                -SIZE, -SIZE,  SIZE,
                SIZE, -SIZE, -SIZE,
                SIZE, -SIZE, -SIZE,
                -SIZE, -SIZE,  SIZE,
                SIZE, -SIZE,  SIZE
            ];
            let raw_model = self.loader.load_simple_model_to_vao(&positions, 3);
            self.skybox_model = Some(SkyboxModel {
                raw_model,
                day_texture_id,
                night_texture_id,
            });
        }
    }

    pub fn init_water(&mut self) {
        if let None = self.water_model {
            let positions = vec![
                -1.0, 0.0, 1.0, 
                1.0, 0.0, 1.0, 
                -1.0, 0.0, -1.0, 
                1.0, 0.0, -1.0, 
            ];
            let raw_model = self.loader.load_simple_model_to_vao(&positions, 3);
            let dudv_tex_id = self.loader.load_terrain_texture("res/textures/water/waterDUDV.png", 0).tex_id;
            let normal_map_tex_id = self.loader.load_terrain_texture("res/textures/water/normalMap.png", 0).tex_id;
            self.water_model = Some(WaterModel {
                raw_model,
                dudv_tex_id,
                normal_map_tex_id,
            });
        }
    }

    pub fn water_model(&self) -> WaterModel {
        self.water_model.clone().expect("Need to call init_water first")
    }
}
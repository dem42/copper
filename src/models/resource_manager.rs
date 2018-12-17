use super::loader::{
    ModelLoader,
    TexturedModel,
    TerrainTexture,  
    TerrainTexturePack,
    RawModel,
};
use crate::entities::Terrain;
use crate::obj_converter::load_obj_model;


#[derive(Default)]
pub struct ResourceManager {
    loader: ModelLoader,
    tree_model: Option<TexturedModel>,
    fern_model: Option<TexturedModel>,
    grass_model: Option<TexturedModel>,
    flowers_model: Option<TexturedModel>,
    low_poly_tree_model: Option<TexturedModel>,
    player_model: Option<TexturedModel>,
    texture_pack: Option<TerrainTexturePack>,
    blend_texture: Option<TerrainTexture>,
    terrain_model: Option<RawModel>,
}

impl ResourceManager {

    pub fn init_player_model(&mut self) {    
        if let None = self.player_model {
            let model_data = load_obj_model("res/models/stanfordBunny.obj").expect("Unable to load stanfordBunny.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let texture = self.loader.load_texture("res/textures/brown.png", false);
            self.player_model = Some(TexturedModel { raw_model, texture });
        } 
    }

    pub fn player_model(&self) -> &TexturedModel {
        self.player_model.as_ref().expect("Need to call init_player_model before accessing the model")
    }

    pub fn init_tree_model(&mut self) {    
        if let None = self.tree_model {
            let model_data = load_obj_model("res/models/tree.obj").expect("Unable to load tree.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let texture = self.loader.load_texture("res/textures/tree.png", false);
            self.tree_model = Some(TexturedModel { raw_model, texture });
        } 
    }

    pub fn tree_model(&self) -> &TexturedModel {
        self.tree_model.as_ref().expect("Need to call init_tree_model before accessing the model")
    }

     pub fn init_low_poly_tree_model(&mut self) {    
        if let None = self.low_poly_tree_model {
            let model_data = load_obj_model("res/models/lowPolyTree.obj").expect("Unable to load lowPolyTree.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let texture = self.loader.load_texture("res/textures/lowPolyTree.png", false);
            self.low_poly_tree_model = Some(TexturedModel { raw_model, texture });
        } 
    }

    pub fn low_poly_tree_model(&self) -> &TexturedModel {
        self.low_poly_tree_model.as_ref().expect("Need to call init_low_poly_tree_model before accessing the model")
    }

    pub fn init_fern_model(&mut self) {    
        if let None = self.fern_model {
            let model_data = load_obj_model("res/models/fern.obj").expect("Unable to load fern.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let mut texture = self.loader.load_texture("res/textures/fern.png", false);
            texture.has_transparency = true;    
            self.fern_model = Some(TexturedModel { raw_model, texture });
        }
    }

    pub fn fern_model(&self) -> &TexturedModel {
        self.fern_model.as_ref().expect("Need to call init_fern_model before accessing the model")
    }

    pub fn init_grass_model(&mut self) {    
        if let None = self.grass_model {
            let model_data = load_obj_model("res/models/grassModel.obj").expect("Unable to load grassModel.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let mut texture = self.loader.load_texture("res/textures/grassTexture.png", false);
            texture.has_transparency = true;
            texture.uses_fake_lighting = true;
            self.grass_model = Some(TexturedModel { raw_model, texture });
        }
    }

    pub fn grass_model(&self) -> &TexturedModel {
        self.grass_model.as_ref().expect("Need to call init_grass_model before accessing the model")
    }

    pub fn init_flowers_model(&mut self) {    
        if let None = self.flowers_model {
            let model_data = load_obj_model("res/models/grassModel.obj").expect("Unable to load grassModel.obj");
            let raw_model = self.loader.load_to_vao(&model_data.vertices, &model_data.texture_coords, &model_data.indices, &model_data.normals);
            let mut texture = self.loader.load_texture("res/textures/flower.png", false);
            texture.has_transparency = true;
            texture.uses_fake_lighting = true;
            self.flowers_model = Some(TexturedModel { raw_model, texture });
        }
    }

    pub fn flowers_model(&self) -> &TexturedModel {
        self.flowers_model.as_ref().expect("Need to call init_flower_model before accessing the model")
    }

    

    pub fn init_terrain_textures(&mut self) {
        if let None = self.texture_pack {
            let background_texture = self.loader.load_terrain_texture("res/textures/terrain/grassy2.png", false);
            let r_texture = self.loader.load_terrain_texture("res/textures/terrain/mud.png", false);
            let g_texture = self.loader.load_terrain_texture("res/textures/terrain/grassFlowers.png", false);
            let b_texture = self.loader.load_terrain_texture("res/textures/terrain/path.png", false);
            self.texture_pack = Some(TerrainTexturePack { background_texture, r_texture, g_texture, b_texture, });
        }
        if let None = self.blend_texture {
            self.blend_texture = Some(self.loader.load_terrain_texture("res/textures/terrain/blendMap.png", false));
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
            let model = Terrain::generate_terrain(&mut self.loader);
            self.terrain_model = Some(model);
        }
    }

    pub fn terrain_model(&self) -> &RawModel {
        self.terrain_model.as_ref().expect("Need to call init_terrain_model before accessing the model")
    }
}
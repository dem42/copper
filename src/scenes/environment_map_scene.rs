use super::scene::Scene;

use crate::display::framebuffers::FboMap;
use crate::entities::{
    Entity,
    Camera,
    Light,
    Player,
    Ground,
    Skybox,
    Terrain,
    DebugEntity,
};
use crate::guis::GuiPanel;
use crate::math::{Vector3f, Vector2f};
use crate::models::{
    ResourceManager,
    Models,
    ModelType,
    TextureId,
};

pub fn init_scene_resources(resource_manager: &mut ResourceManager) {
    resource_manager.init(&Models::PLAYER);
    resource_manager.init(&Models::DRAGON);
    resource_manager.init(&Models::META);
    resource_manager.init(&Models::TEA);
    
    resource_manager.init_terrain_textures();
    resource_manager.init_terrain_model();

    resource_manager.init_cathedral_skybox();
    
    resource_manager.init_quad_model();

    // debug entity
    resource_manager.init_debug_cuboid_model();
}

pub fn create_scene(resource_manager: &mut ResourceManager, _framebuffers: &FboMap) -> Scene {
    let entities = Vec::new();
    
    let terrains = Vec::new();   
    let ground = Ground { terrains };

    let player_entity = Entity::new(resource_manager.model(ModelType::Player), Vector3f::new(0.0, 0.0, 0.0), Vector3f::new(0.0, 0.0, 0.0), 1.0);
    let mut player = Player::new(player_entity);
    player.is_invisible_immovable = true;
    
    let water_tiles = Vec::new();
    let normal_mapped_entities = Vec::new();

    let mut debug_entity = DebugEntity::new(resource_manager.debug_cuboid_model());
    debug_entity.position.y = 10.0;

    let mut camera = Camera::new(20.0, 50.0);
    camera.position = Vector3f::new(0.0, 0.0, 0.0);

    let mut skybox = Skybox::new(resource_manager.cathedral_skybox(), 0.0);
    skybox.uses_fog = false;

    let texts = Vec::new();
    
    let lights = vec!{
        //Light::new_infinite(Vector3f::new(0.0, 10000.0, 0.0), Vector3f::new(0.8, 0.8, 0.8)), // sunlight, no attenuation
        Light::new_infinite(Vector3f::new(5000.0, 10000.0, -5000.0), Vector3f::new(0.8, 0.8, 0.8)), // sunlight, no attenuation
    };

    let particle_systems = Vec::new();

    let guis = Vec::new();

    let entities_with_env_map = vec![
        Entity::new(resource_manager.model(ModelType::Dragon), Vector3f::new(-50.0, 20.0, -100.0), Vector3f::new(0.0, 0.0, 0.0), 1.0),
        Entity::new(resource_manager.model(ModelType::Tea), Vector3f::new(0.0, 20.0, -100.0), Vector3f::new(0.0, 0.0, 0.0), 1.0),
        Entity::new(resource_manager.model(ModelType::Meta), Vector3f::new(50.0, 20.0, -100.0), Vector3f::new(0.0, 0.0, 0.0), 1.0),
    ];

    Scene {
        entities, 
        normal_mapped_entities, 
        ground, 
        player, 
        quad_model: resource_manager.quad_model(), 
        water: water_tiles,
        debug_entity,
        camera,
        skybox,
        texts,
        guis,
        lights,
        particle_systems,
        uses_post_processing: false,
        entities_with_env_map,
    }
}
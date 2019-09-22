use super::scene::Scene;

use crate::display::framebuffers::FboMap;
use crate::entities::{
    Entity,
    Camera,
    Light,
    Player,
    Ground,
    Skybox,
    DebugEntity,
};
use crate::math::{Vector3f};
use crate::models::{
    ResourceManager,
    Models,
    ModelType,
    SkyboxModel,
    RawModel,
    TextureId,
};
use crate::particles::{
    AdvancedParticleSystem,
    ParticleSystemProps,
};

pub fn init_scene_resources(resource_manager: &mut ResourceManager) {
    resource_manager.init(&Models::PLAYER);
    
    resource_manager.init_quad_model();

    // debug entity
    resource_manager.init_debug_cuboid_model();

    resource_manager.init_simple_point_particle_model();
    resource_manager.init_particle_model();
    resource_manager.init_particle_textures();
}

// to use the particles in this example you need to use the correct renderer in the ParticleMaster
// at the moment no switching through parameters to simplify things since gpu instancing is more efficient anyway
pub fn create_scene(resource_manager: &mut ResourceManager, _framebuffers: &FboMap) -> Scene {    
    let entities = Vec::new();
    
    let terrains = Vec::new();
    let ground = Ground { terrains };

    //let player_entity = Entity::new(resource_manager.model(ModelType::Player), ground.create_pos_on_terrain(150.0, -250.0), Vector3f::new(0.0, 180.0, 0.0), 0.3);
    let player_entity = Entity::new(resource_manager.model(ModelType::Player), Vector3f::new(0.0, 0.0, 0.0), Vector3f::new(0.0, 180.0, 0.0), 0.3);
    let mut player = Player::new(player_entity);
    player.is_invisible_immovable = true;
    
    let water_tiles = Vec::new();
    let normal_mapped_entities = Vec::new();

    let mut debug_entity = DebugEntity::new(resource_manager.debug_cuboid_model());
    debug_entity.position.y = 10.0;

    let mut camera = Camera::new(20.0, 30.0);
    camera.position = Vector3f::new(0.0, 0.0, 0.0);

    let skybox = Skybox::new(SkyboxModel {raw_model: RawModel {vao_id: 0, vertex_count: 0}, day_texture_id: TextureId::Empty, night_texture_id: TextureId::Empty, cycles_day_night: false}, 0.0);

    let texts = Vec::new();
    
    let lights = vec!{
        //Light::new_infinite(Vector3f::new(0.0, 10000.0, 0.0), Vector3f::new(0.8, 0.8, 0.8)), // sunlight, no attenuation
        Light::new_infinite(Vector3f::new(5000.0, 10000.0, -5000.0), Vector3f::new(0.8, 0.8, 0.8)), // sunlight, no attenuation
    };

    let particle_spawn = player.position().clone();    
    let particle_system = AdvancedParticleSystem::new(resource_manager.simple_point_particle_model(), resource_manager.particle_texture(ResourceManager::SMOKE_ATLAS),
        ParticleSystemProps { 
            particles_per_sec: 50.0, speed: 15.0, scale: 6.5, 
            gravity_effect: 0.5, life_length: 1.5, 
            speed_error: 0.3, life_error: 0.3, scale_error: 0.1, 
            randomize_rotation: true, direction: Some((Vector3f::new(0.0, 1.0, 0.0), 150.0)),
            additive_blending: false,
        }
    );

    let particle_systems = vec![        
        (particle_system, particle_spawn),
    ];
    
    let guis = Vec::new();

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
        entities_with_env_map: Vec::new(),
    }
}
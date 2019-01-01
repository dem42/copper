extern crate copper;
extern crate rand;

use rand::Rng;

use copper::display::Display;
use copper::renderers::{
    BatchRenderer,
};
use copper::models::{
    ResourceManager,
};
use copper::entities::{
    Entity,
    Camera,
    Light,
    Terrain,
    Player,
};
use copper::math::Vector3f;

fn main() {
    let mut display = Display::create();
    let mut resource_manager = ResourceManager::default();
    
    let (entities, terrains, mut player) = create_world(&mut resource_manager);

    let mut batch_renderer = BatchRenderer::new(&display);
    
    let light = Light::new(Vector3f::new(200.0,200.0,100.0), Vector3f::new(1.0, 1.0, 1.0));

    let mut camera = Camera::new();
    camera.position = Vector3f::new(0.0, 10.0, 5.0);
    
    while !display.is_close_requested() {
        camera.move_camera(&display, &player);
        player.move_player(&display);     
        batch_renderer.render(&light, &camera, &entities, &terrains, &player);
        display.update_display();
    }
}

fn create_world(resource_manager: &mut ResourceManager) -> (Vec<Entity>, Vec<Terrain>, Player) {
    let mut entities = Vec::new();    
    let mut rng = rand::thread_rng();
    const X_WIDTH: f32 = 1000.0;
    const Z_WIDTH: f32 = -1000.0;
    resource_manager.init_tree_model();
    resource_manager.init_fern_model();
    resource_manager.init_grass_model();
    resource_manager.init_flowers_model();
    resource_manager.init_low_poly_tree_model();
    resource_manager.init_terrain_textures();
    resource_manager.init_terrain_model();
    resource_manager.init_player_model();

    for _ in 0..100 {
        let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(resource_manager.tree_model(), r_pos, r_rot, 3.0));

        let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(resource_manager.low_poly_tree_model(), r_pos, r_rot, 0.5));

        let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.fern_model(), r_pos, r_rot, 0.6));

        let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.grass_model(), r_pos, r_rot, 1.0));

        let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.flowers_model(), r_pos, r_rot, 1.0));
    }    

    let mut terrains = Vec::new();
    
    for i in -2..2 {
        for j in -2..2 {            
            let terrain = Terrain::new(i, j, resource_manager.terrain_pack(), resource_manager.blend_texture(), resource_manager.terrain_model());
            terrains.push(terrain);
        }
    }

    let player_entity = Entity::new(resource_manager.player_model(), Vector3f::new(0.0, 0.0, -50.0), Vector3f::new(0.0, 180.0, 0.0), 1.0);
    let player = Player::new(player_entity);

    (entities, terrains, player)
}
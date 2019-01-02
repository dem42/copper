extern crate copper;
extern crate rand;

use rand::Rng;

use copper::display::Display;
use copper::guis::Gui;
use copper::renderers::{
    BatchRenderer,
    GuiRenderer,
};
use copper::models::{
    ResourceManager,
    Models,
    ModelType,
    GuiModel,
};
use copper::entities::{
    Entity,
    Camera,
    Light,
    Terrain,
    Player,
    Ground,
};
use copper::math::{
    Vector2f,
    Vector3f,
};

fn main() {
    let mut display = Display::create();
    let mut resource_manager = ResourceManager::default();
    
    resource_manager.init_gui_textures();
    // hmm odd that this is ok. doesnt seem to count as a borrow for primitives
    let healthbar = resource_manager.get_gui_texture(ResourceManager::HEALTHBAR_TEXTURE);
    let gui_background = resource_manager.get_gui_texture(ResourceManager::GUI_BACKGROUND_TEXTURE);

    let (entities, ground, mut player, gui_model) = create_world(&mut resource_manager);
    let guis = vec!{
        Gui::new(gui_background, Vector2f::new(-0.8, -0.9), Vector2f::new(0.4, 0.4)),
        Gui::new(healthbar, Vector2f::new(-0.75, -0.9), Vector2f::new(0.3, 0.3)),
    };

    let mut batch_renderer = BatchRenderer::new(&display);
    let mut gui_renderer = GuiRenderer::new();
    
    let light = Light::new(Vector3f::new(200.0,200.0,100.0), Vector3f::new(1.0, 1.0, 1.0));

    let mut camera = Camera::new();
    camera.position = Vector3f::new(0.0, 10.0, 5.0);
    
    while !display.is_close_requested() {
        camera.move_camera(&display, &player);
        player.move_player(&display, &ground);     
        batch_renderer.render(&light, &camera, &entities, &ground.terrains, &player);
        gui_renderer.render(&guis, &gui_model.raw_model);
        display.update_display();
    }
}

fn create_world(resource_manager: &mut ResourceManager) -> (Vec<Entity>, Ground, Player, &GuiModel) {
    let mut entities = Vec::new();    
    let mut rng = rand::thread_rng();
    const X_WIDTH: f32 = 1000.0;
    const Z_WIDTH: f32 = -1000.0;
    resource_manager.init(&Models::TREE);
    resource_manager.init(&Models::FERN);
    resource_manager.init(&Models::GRASS);
    resource_manager.init(&Models::FLOWERS);
    resource_manager.init(&Models::LOW_POLY_TREE);
    resource_manager.init_terrain_textures();
    resource_manager.init_terrain_model();
    resource_manager.init(&Models::PLAYER);
    resource_manager.init(&Models::CRATE);
    resource_manager.init_gui_model();
    
    let mut terrains = Vec::new();    
    for i in -2..2 {
        for j in -2..2 {            
            let terrain = Terrain::new(i, j, resource_manager.terrain_pack(), resource_manager.blend_texture(), resource_manager.terrain_model());
            terrains.push(terrain);
        }
    }
    let ground = Ground { terrains };

    for _ in 0..100 {
        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::Tree), r_pos, r_rot, 3.0));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::LowPolyTree), r_pos, r_rot, 0.5));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        let fern_model = resource_manager.model(ModelType::Fern);
        let atlas_texture_index: usize = rng.gen_range(0, fern_model.texture.number_of_rows_in_atlas * fern_model.texture.number_of_rows_in_atlas);
        entities.push(Entity::new_with_texture_atlas(fern_model, r_pos, r_rot, 0.6, atlas_texture_index));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::Grass), r_pos, r_rot, 1.0));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::Flowers), r_pos, r_rot, 1.0));
    }    

    let player_entity = Entity::new(resource_manager.model(ModelType::Player), ground.create_pos_on_terrain(0.0, -50.0), Vector3f::new(0.0, 180.0, 0.0), 0.3);
    let player = Player::new(player_entity);

    let mut box_pos = ground.create_pos_on_terrain(0.0, -150.0);
    box_pos.y += 4.0;
    let box_entity = Entity::new(resource_manager.model(ModelType::Crate), box_pos, Vector3f::new(0.0, 0.0, 0.0), 5.0);
    entities.push(box_entity);

    (entities, ground, player, resource_manager.gui_model())
}
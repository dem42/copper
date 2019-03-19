extern crate copper;
extern crate rand;

use rand::Rng;

use copper::display::{
    Display,
    Framebuffers,
};
use copper::guis::{
    GuiPanel,
    TextMaterial,
};
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
    Skybox,
    WaterTile,
};
use copper::math::{
    Vector2f,
    Vector3f,
};
use copper::mouse_picker::MousePicker;

struct Scene {
    entities: Vec<Entity>, 
    normal_mapped_entities: Vec<Entity>, 
    ground: Ground, 
    player: Player, 
    gui_model: GuiModel, 
    water: Vec<WaterTile>,
}

fn main() {
    let mut display = Display::create();
    let framebuffers = Framebuffers::new(&display);
    let mut resource_manager = ResourceManager::default();

    init_resources(&mut resource_manager);
    
    let texts = vec![
        resource_manager.create_gui_text("peljo'\nwo rld!", 
            ResourceManager::COPPER_SDF_FONT_TYPE, 10, Vector2f::new(0.0, 0.5), 
            TextMaterial {
                color: Vector3f::new(1.0, 0.0, 0.0), ..TextMaterial::default()
            }
        ),
        resource_manager.create_gui_text("Not_imPres\" Sejp; @", 
            ResourceManager::COPPER_SDF_FONT_TYPE, 15, Vector2f::new(-0.5, -0.5), 
            TextMaterial {
                color: Vector3f::new(0.0, 0.0, 1.0), 
                outline_color: Vector3f::new(0.2, 0.2, 0.2),
                offset: Vector2f::new(-0.002, -0.002),
                ..TextMaterial::default()
            }
        ),
    ];

    let mut scene = create_scene(&resource_manager);
    let healthbar = resource_manager.get_gui_texture(ResourceManager::HEALTHBAR_TEXTURE);
    let gui_background = resource_manager.get_gui_texture(ResourceManager::GUI_BACKGROUND_TEXTURE);
    let guis = vec!{
        GuiPanel::new(gui_background, Vector2f::new(-0.73, -0.7), Vector2f::new(0.25, 0.25)),
        GuiPanel::new(healthbar, Vector2f::new(-0.75, -0.75), Vector2f::new(0.2, 0.2)),
    };


    let mut batch_renderer = BatchRenderer::new(&display);
    let mut gui_renderer = GuiRenderer::new();
        
    let mut lights = vec!{
        Light::new_infinite(Vector3f::new(0.0,10_000.0,-7_000.0), Vector3f::new(0.8, 0.8, 0.8)), // sunlight, no attenuation
        Light::new_point(scene.ground.create_pos_above_terrain(185.0,12.5,-293.0), Vector3f::new(2.0, 0.0, 0.0), Vector3f::new(1.0, 0.01, 0.002)),
        Light::new_point(scene.ground.create_pos_above_terrain(370.0,14.0,-300.0), Vector3f::new(0.0, 2.0, 2.0), Vector3f::new(1.0, 0.01, 0.002)),
        Light::new_point(scene.ground.create_pos_above_terrain(293.0,14.0,-305.0), Vector3f::new(2.0, 2.0, 0.0), Vector3f::new(1.0, 0.01, 0.002)),        
    };
    // add lamps 
    scene.entities.push(Entity::new(resource_manager.model(ModelType::Lamp), scene.ground.create_pos_on_terrain(185.0, -293.0), Vector3f::new(0.0, 0.0, 0.0), 1.0));
    scene.entities.push(Entity::new(resource_manager.model(ModelType::Lamp), scene.ground.create_pos_on_terrain(370.0, -300.0), Vector3f::new(0.0, 0.0, 0.0), 1.0));
    scene.entities.push(Entity::new(resource_manager.model(ModelType::Lamp), scene.ground.create_pos_on_terrain(293.0, -305.0), Vector3f::new(0.0, 0.0, 0.0), 1.0));

    let mut camera = Camera::new();
    camera.position = Vector3f::new(0.0, 10.0, 5.0);

    let mut skybox = Skybox::new(resource_manager.skybox(), 0.0);

    let mut mouse_picker = MousePicker::new();
    
    while !display.is_close_requested() {
        camera.move_camera(&display, &scene.player);
        if let Some(selected_pos) = mouse_picker.update(&display, &batch_renderer.projection_matrix, &camera, &scene.ground) {            
            let last_pos = scene.entities.len()-1;
            scene.entities[last_pos].set_position(&selected_pos);
            lights[3].position = selected_pos;
            lights[3].position.y += 14.0; 
        }

        const SPEED: f32 = 20.0;
        for idx in 0..scene.normal_mapped_entities.len() {
            scene.normal_mapped_entities[idx].increase_rotation(0.0, SPEED * display.frame_time_sec, 0.0);
        }

        scene.player.move_player(&display, &scene.ground);
        skybox.increase_rotation(&display);
        batch_renderer.render(&lights, &mut camera, &scene.entities, &scene.normal_mapped_entities, &scene.ground.terrains, 
            &scene.player, &scene.water, &skybox, &display, &framebuffers);
        gui_renderer.render(&guis, &scene.gui_model.raw_model, &texts);
        display.update_display();
    }
}

fn init_resources(resource_manager: &mut ResourceManager) {
    //resource_manager.init(&Models::TREE);
    resource_manager.init(&Models::FERN);
    //resource_manager.init(&Models::GRASS);
    //resource_manager.init(&Models::FLOWERS);
    resource_manager.init(&Models::TOON_ROCKS);
    resource_manager.init(&Models::BOBBLE_TREE);
    resource_manager.init(&Models::LOW_POLY_TREE);
    resource_manager.init(&Models::PLAYER);
    resource_manager.init(&Models::CRATE);
    resource_manager.init(&Models::LAMP);
    resource_manager.init(&Models::BARREL);
    resource_manager.init(&Models::BOULDER);

    resource_manager.init_terrain_textures();
    resource_manager.init_terrain_model();

    resource_manager.init_skybox();
    resource_manager.init_water();

    resource_manager.init_gui_model();
    resource_manager.init_gui_textures();

    resource_manager.init_fonts();
}

fn create_scene(resource_manager: &ResourceManager) -> Scene {
    let mut entities = Vec::new();    
    let mut rng = rand::thread_rng();
    const X_WIDTH: f32 = 1000.0;
    const Z_WIDTH: f32 = -1000.0;    
    
    let mut terrains = Vec::new();    
    for i in -2..2 {
        for j in -2..2 {            
            let terrain = Terrain::new(i, j, resource_manager.terrain_pack(), resource_manager.blend_texture(), resource_manager.terrain_model());
            terrains.push(terrain);
        }
    }
    let ground = Ground { terrains };

    for _ in 0..100 {
        // let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        // let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        // entities.push(Entity::new(resource_manager.model(ModelType::Tree), r_pos, r_rot, 3.0));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::LowPolyTree), r_pos, r_rot, 0.5));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        let fern_model = resource_manager.model(ModelType::Fern);
        let atlas_texture_index: usize = rng.gen_range(0, fern_model.texture.number_of_rows_in_atlas * fern_model.texture.number_of_rows_in_atlas);
        entities.push(Entity::new_with_texture_atlas(fern_model, r_pos, r_rot, 0.6, atlas_texture_index));

        // let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        // let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        // entities.push(Entity::new(resource_manager.model(ModelType::Grass), r_pos, r_rot, 1.0));

        // let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        // let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        // entities.push(Entity::new(resource_manager.model(ModelType::Flowers), r_pos, r_rot, 1.0));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::BobbleTree), r_pos, r_rot, 0.5));

        let r_pos = ground.create_pos_on_terrain(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, rng.gen::<f32>() * Z_WIDTH);
        let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
        entities.push(Entity::new(resource_manager.model(ModelType::ToonRocks), r_pos, r_rot, 1.0));
    }    

    let player_entity = Entity::new(resource_manager.model(ModelType::Player), ground.create_pos_on_terrain(150.0, -250.0), Vector3f::new(0.0, 180.0, 0.0), 0.3);
    let player = Player::new(player_entity);

    let mut box_pos = ground.create_pos_on_terrain(0.0, -150.0);
    box_pos.y += 4.0;
    let box_entity = Entity::new(resource_manager.model(ModelType::Crate), box_pos, Vector3f::new(0.0, 0.0, 0.0), 5.0);
    entities.push(box_entity);

    let water_tiles = vec![
        // put the water slightly below 0 to reduce z-fighting since a lot of terrain is at 0
        WaterTile::new(Vector3f::new(150.0, -0.2, -250.0), resource_manager.water_model()),
    ];

    let mut normal_mapped_entities = Vec::new();   
    normal_mapped_entities.push(Entity::new(resource_manager.model(ModelType::Barrel), ground.create_pos_above_terrain(150.0, 10.0, -255.0), Vector3f::zero(), 0.5));
    normal_mapped_entities.push(Entity::new(resource_manager.model(ModelType::Boulder), ground.create_pos_above_terrain(140.0, 10.0, -255.0), Vector3f::zero(), 0.5));

    Scene {
        entities, 
        normal_mapped_entities, 
        ground, 
        player, 
        gui_model: resource_manager.gui_model(), 
        water: water_tiles
    }
}
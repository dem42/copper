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
};
use copper::math::Vector3f;

fn main() {
    let mut display = Display::create();
    let mut resource_manager = ResourceManager::default();
    
    let (entities, terrains) = create_world(&mut resource_manager);

    let mut batch_renderer = BatchRenderer::new(&display);
    
    let light = Light::new(Vector3f::new(200.0,200.0,100.0), Vector3f::new(1.0, 1.0, 1.0));

    let mut camera = Camera::default();
    camera.position.y = 5.0;
    
    while !display.is_close_requested() {
        camera.move_camera(&display);        
        batch_renderer.render(&light, &camera, &entities, &terrains);
        display.update_display();
    }
}

fn create_world<'a>(resource_manager: &'a mut ResourceManager) -> (Vec<Entity<'a>>, Vec<Terrain<'a>>) {
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
    (entities, terrains)
}

// fn create_world<'a>(tree_model: &'a TexturedModel, fern_model: &'a TexturedModel, grass_model: &'a TexturedModel) -> Vec<Entity<'a>> {
//     let mut entities = Vec::new();    
//     let mut rng = rand::thread_rng();
//     const X_WIDTH: f32 = 1000.0;
//     const Z_WIDTH: f32 = -1000.0;
//     for _ in 0..100 {
//         let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
//         let r_rot = Vector3f::new(0.0, 0.0, 0.0);
//         entities.push(Entity::new(&tree_model, r_pos, r_rot, 3.0));

//         let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
//         let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
//         entities.push(Entity::new(&fern_model, r_pos, r_rot, 0.6));

//         let r_pos = Vector3f::new(rng.gen::<f32>() * X_WIDTH - X_WIDTH/2.0, 0.0, rng.gen::<f32>() * Z_WIDTH);
//         let r_rot = Vector3f::new(0.0, rng.gen::<f32>() * 180.0, 0.0);
//         entities.push(Entity::new(&grass_model, r_pos, r_rot, 1.0));
//     }
//     entities
// }


// fn cubes<'a>(textured_model: &'a TexturedModel) -> Vec<Entity<'a>> {
//     let mut entities = Vec::new();
//     let mut rng = rand::thread_rng();
//     for _ in 0..200 {
//         let r_pos = Vector3f::new(rng.gen::<f32>() * 100.0 - 50.0, rng.gen::<f32>() * 100.0 - 50.0, rng.gen::<f32>() * -300.0);
//         let r_rot = Vector3f::new(rng.gen::<f32>() * 180.0, rng.gen::<f32>() * 180.0, 0.0);
//         entities.push(Entity::new(&textured_model, r_pos, r_rot, 1.0));
//     }
//     entities
// }

// fn cube_model(loader: &mut ModelLoader) -> TexturedModel {
//     let raw_model = load_obj_model("res/models/cube.obj", loader).expect("Unable to load cube.obj");
//     let texture = loader.load_texture("res/textures/rainbow512.png", 1.0, 0.1, false);
//     TexturedModel { raw_model, texture }
// }

// fn blue_dragon(loader: &mut ModelLoader) -> TexturedModel {
//     let raw_model = load_obj_model("res/models/DragonBlender.obj", loader).expect("Unable to load dragon .obj");
//     let texture = loader.load_texture("res/textures/dragon_texture.png", 10.0, 1.0, false);
//     TexturedModel { raw_model, texture }
// }

// fn dragon(loader: &mut ModelLoader) -> TexturedModel {
//     let raw_model = load_obj_model("res/models/dragon.obj", loader).expect("Unable to load dragon .obj");
//     let texture = loader.load_texture("res/textures/white.png", 10.0, 1.0, false);
//     TexturedModel { raw_model, texture }
// }

// fn stall_model(loader: &mut ModelLoader) -> TexturedModel {
//     let raw_model = load_obj_model("res/models/stall_textured.obj", loader).expect("Failed to load stall.obj model");
//     let texture = loader.load_texture("res/textures/stallTexture.png", 1.0, 0.0, false);
//     TexturedModel { raw_model, texture }
// }

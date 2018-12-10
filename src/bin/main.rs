extern crate copper;
extern crate rand;

use rand::Rng;

use copper::display::Display;
use copper::renderers::{
    BatchRenderer,
};
use copper::loader::{
    ModelLoader,
    TexturedModel,
    ModelTexture,  
};
use copper::entities::{
    Entity,
    Camera,
    Light,
    Terrain,
};
use copper::math::Vector3f;
use copper::obj_loader::load_obj_model;

fn test_engine() {
    let mut display = Display::create();    
    let mut loader = ModelLoader::new();

    let textured_model = tree_model(&mut loader);
    let entities = trees(&textured_model);

    let mut batch_renderer = BatchRenderer::new(&display);
    
    let light = Light::new(Vector3f::new(200.0,200.0,100.0), Vector3f::new(1.0, 1.0, 1.0));

    let mut camera = Camera::default();
    camera.position.y = 10.0;

    let grass_tex = loader.load_texture("res/textures/grass.png", 1.0, 0.0, false);
    let terrains = terrain_cells(&grass_tex, &mut loader);

    while !display.is_close_requested() {
        camera.move_camera(&display);        
        batch_renderer.render(&light, &camera, &entities, &terrains);
        display.update_display();
    }
}

fn terrain_cells<'a>(grass_tex: &'a ModelTexture, loader: &mut ModelLoader) -> Vec<Terrain<'a>> {
    let mut terrains = Vec::new();
    for i in -2..2 {
        for j in -2..2 {
            let terrain = Terrain::new(i, j, grass_tex, loader);
            terrains.push(terrain);
        }
    }
    terrains
}

fn trees<'a>(textured_model: &'a TexturedModel) -> Vec<Entity<'a>> {
    let mut entities = Vec::new();    
    let mut rng = rand::thread_rng();
    for _ in 0..200 {
        let r_pos = Vector3f::new(rng.gen::<f32>() * 100.0 - 50.0, 0.0, rng.gen::<f32>() * -300.0);
        let r_rot = Vector3f::new(0.0, 0.0, 0.0);
        entities.push(Entity::new(&textured_model, r_pos, r_rot, 1.0));
    }
    entities
}


fn tree_model(loader: &mut ModelLoader) -> TexturedModel {
    let raw_model = load_obj_model("res/models/tree.obj", loader).expect("Unable to load tree.obj");
    let texture = loader.load_texture("res/textures/tree.png", 1.0, 0.1, false);
    TexturedModel { raw_model, texture }
}


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

fn main() {
    test_engine();    
}

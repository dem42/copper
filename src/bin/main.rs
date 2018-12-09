extern crate copper;
extern crate rand;

use rand::Rng;

use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    TexturedModel,    
};
use copper::shaders::shader_program::{
    StaticShader,
};
use copper::entities::{
    Entity,
    Camera,
    Light,
};
use copper::math::Vector3f;
use copper::obj_loader::load_obj_model;

fn test_engine() {
    let mut display = Display::create();    
    let mut loader = ModelLoader::new();

    let textured_model = cube(&mut loader);
    let mut shader = StaticShader::new(&textured_model.raw_model);

    let renderer = Renderer::new(&display, &mut shader);

    //let entity = Entity::new(&textured_model, Vector3f::new(0.0,0.0,-25.0), Vector3f::new(0.0, 0.0, 0.0), 1.0);
    let mut entites = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..200 {
        let r_pos = Vector3f::new(rng.gen::<f32>() * 100.0 - 50.0, rng.gen::<f32>() * 100.0 - 50.0, rng.gen::<f32>() * -300.0);
        let r_rot = Vector3f::new(rng.gen::<f32>() * 180.0, rng.gen::<f32>() * 180.0, 0.0);
        entites.push(Entity::new(&textured_model, r_pos, r_rot, 1.0));
    }

    let light = Light::new(Vector3f::new(200.0,200.0,100.0), Vector3f::new(1.0, 1.0, 1.0));

    let mut camera = Camera::default();

    while !display.is_close_requested() {        
        camera.move_camera(&display);

        renderer.prepare();
        shader.start();
        shader.load_light(&light);
        shader.load_view_matrix(&camera);
        for entity in entites.iter() {
            renderer.render(&entity, &mut shader);
        }
        shader.stop();
        display.update_display();
    }
}

fn cube(loader: &mut ModelLoader) -> TexturedModel {
    let raw_model = load_obj_model("res/models/cube.obj", loader).expect("Unable to load cube.obj");
    let texture = loader.load_texture("res/textures/rainbow512.png", 1.0, 0.1, false);
    TexturedModel { raw_model, texture }
}

fn blue_dragon(loader: &mut ModelLoader) -> TexturedModel {
    let raw_model = load_obj_model("res/models/DragonBlender.obj", loader).expect("Unable to load dragon .obj");
    let texture = loader.load_texture("res/textures/dragon_texture.png", 10.0, 1.0, false);
    TexturedModel { raw_model, texture }
}

fn dragon(loader: &mut ModelLoader) -> TexturedModel {
    let raw_model = load_obj_model("res/models/dragon.obj", loader).expect("Unable to load dragon .obj");
    let texture = loader.load_texture("res/textures/white.png", 10.0, 1.0, false);
    TexturedModel { raw_model, texture }
}

fn stall_model(loader: &mut ModelLoader) -> TexturedModel {
    let raw_model = load_obj_model("res/models/stall_textured.obj", loader).expect("Failed to load stall.obj model");
    let texture = loader.load_texture("res/textures/stallTexture.png", 1.0, 0.0, false);
    TexturedModel { raw_model, texture }
}

fn main() {
    test_engine();    
}

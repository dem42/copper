extern crate copper;
use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    TexturedModel,    
};
use copper::shaders::shader_program::{
    StaticShader,
};
use copper::entities::Entity;
use copper::math::Vector3f;

fn test_engine() {
    let mut display = Display::create();
    let renderer = Renderer::new();
    let mut loader = ModelLoader::new();
    
    let vertices = vec![
        -0.5, 0.5, 0.0, //v0
        -0.5, -0.5, 0.0, //v1
        0.5, -0.5, 0.0, //v2
        0.5, 0.5, 0.0,  //v3      
    ];
    let tex_coords = vec![
        0.0, 1.0, //v0
        0.0, 0.0, //v1
        1.0, 0.0, //v2
        1.0, 1.0, //v3
    ];
    let indices = vec![
        0,1,3,
        3,1,2,
    ];
    let raw_model = loader.load_to_vao(&vertices, &tex_coords, &indices);
    let texture = loader.load_texture("res/textures/test.png");
    let textured_model = TexturedModel { raw_model, texture };
    let shader = StaticShader::new(&textured_model.raw_model);
    let mut entity = Entity::new(textured_model, Vector3f::new(-1.0,0.0,0.0), Vector3f::new(0.0, 0.0, 0.0), 1.0);

    while !display.is_close_requested() {
        entity.increase_position(0.002, 0.0, 0.0);
        entity.increase_rotation(0.0, 1.0, 0.0);
        renderer.prepare();
        shader.start();
        renderer.render(&entity, &shader);
        shader.stop();
        display.update_display();
    }
}

fn main() {
    test_engine();    
}

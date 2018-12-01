extern crate copper;
use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    TexturedModel,    
};
use copper::shaders::create_static_shader_for_model;

fn test_engine() {
    let mut display = Display::create();
    let renderer = Renderer::new();
    let mut loader = ModelLoader::new();
    
    loader.load_texture();

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
    let texture = loader.load_texture();
    let textured_model = TexturedModel { raw_model, texture };
    let shader = create_static_shader_for_model(&textured_model.raw_model);

    while !display.is_close_requested() {
        renderer.prepare();
        shader.start();
        renderer.render(&textured_model);
        shader.stop();
        display.update_display();
    }
}

fn main() {
    test_engine();    
}

extern crate copper;
use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    RawModel
};
use copper::shaders::create_static_shader_for_model;

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
    let indices = vec![
        0,1,3,
        3,1,2,
    ];
    let model = loader.load_to_vao(&vertices, &indices);
    let shader = create_static_shader_for_model(&model);

    while !display.is_close_requested() {
        renderer.prepare();
        shader.start();
        renderer.render(&model);
        shader.stop();
        display.update_display();
    }
}

fn main() {
    test_engine();    
}

extern crate copper;
use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    RawModel
};

fn test_engine() {
    let mut display = Display::create();
    let renderer = Renderer::new();
    let mut loader = ModelLoader::new();
    
    let vertices = vec![
        -0.5, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,

        0.5, -0.5, 0.0,
        0.5, 0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];
    let model = loader.load_to_vao(&vertices);

    while !display.is_close_requested() {
        renderer.prepare();
        renderer.render(&model);
        display.update_display();
    }
}

fn main() {
    test_engine();    
}

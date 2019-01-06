extern crate texture_lib;

fn main() {
    let img = texture_lib::texture_loader::load_rgba_2d_texture("../res/textures/test.png", false).expect("test");
    println!("???? {}", img.width);
}
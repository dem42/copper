use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

use cmake;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();    
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, ["GL_EXT_texture_filter_anisotropic"])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let dst = cmake::build("libglfw");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    eprintln!("Compiling glfw into: {}", dst.display());
    println!("cargo:rustc-link-lib=static=lglfw3");

    // link with glfw3 which is our dependency (the link library name seems to be specified inside glfw-rs which is a crate we depend on)
    //println!(r"cargo:rustc-link-search=D:\opengl_workspace\glfw-3.3.bin.WIN64\lib-vc2015"); 
}
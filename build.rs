extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gl, (3, 2), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();


    // link with glfw3 which is our dependency (the link library name seems to be specified inside glfw-rs which is a crate we depend on)
    println!(r"cargo:rustc-link-search=D:\opengl_workspace\glfw-3.2.1.bin.WIN64\lib-vc2015"); 
}
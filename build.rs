extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=glut");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(r"-IC:\Program Files (x86)\NVIDIA Corporation\Cg\include\GL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");


    println!(r"cargo:rustc-link-search=D:\opengl_workspace\snappy_test");
    println!(r"cargo:rustc-link-search=D:\opengl_workspace\glew-2.1.0\lib\Release\x64");
    println!(r"cargo:rustc-link-search=C:\Program Files (x86)\NVIDIA Corporation\Cg\lib.x64");
}
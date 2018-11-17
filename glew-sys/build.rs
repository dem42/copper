extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=glew32");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(r"-ID:\opengl_workspace\glew-2.1.0\include\GL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!(r"cargo:rustc-link-search=D:\opengl_workspace\glew-2.1.0\lib\Release\x64");    
}
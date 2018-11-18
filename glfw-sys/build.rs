extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=glfw3");
    println!("cargo:rustc-link-lib=OpenGL32");
    println!("cargo:rustc-link-lib=User32");
    println!("cargo:rustc-link-lib=Gdi32");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(r"-ID:\opengl_workspace\glfw-3.2.1.bin.WIN64\include\GLFW")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!(r"cargo:rustc-link-search=D:\opengl_workspace\glfw-3.2.1.bin.WIN64\lib-vc2015");    
}
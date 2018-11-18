extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=OpenGL32");

    let bindings = bindgen::Builder::default()        
        .header("wrapper.h")
        .clang_arg(r"-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.14393.0\um\gl")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!(r"cargo:rustc-link-search=C:\Program Files (x86)\Windows Kits\10\Lib\10.0.14393.0\um\x64");    
}
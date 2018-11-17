#![allow(dead_code)]
extern crate libc;
use libc::{size_t, c_int, c_uint, c_char};
use std::str;
use std::ffi::CStr;

extern "C" {
    fn abs(input: i32) -> i32;
}

// :O :O :O changing to kind = "static" the library had previously complained about __imp_ which seems to be what you attempt to add when linking against dll?
// note i also needed a release .lib otherwise i was getting errors
#[link(name = "snappy", kind = "static")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

// the glew32 library (wrangler) uses a static lib that then loads the glew32.dll -> in windows this dll needs to be either in PATH or same folder
const GLEW_OK: c_uint = 0;
const GLEW_VERSION: c_uint = 1;
#[link(name = "glew32", kind = "static")]
extern {
    fn glewInit() -> c_uint;
    fn glewGetErrorString(error: c_uint) -> *const c_char;
    fn glewGetString(name: c_uint) -> *const c_char;
}

// the glut library (utilities for creating context) uses a static lib that then loads the glut32.dll -> in windows this dll needs to be either in PATH or same folder
#[link(name = "glut32", kind = "static")]
#[allow(non_snake_case)]
extern "stdcall" {
    fn glutInit(argc: *const c_int, argv: *const *const u8);
    fn glutCreateWindow(title: *const u8) -> c_int;
    fn glutMainLoop();
    fn glutInitWindowSize(width: c_int, height: c_int);
    fn glutInitWindowPosition(x: c_int, y: c_int);
    fn glutDisplayFunc(func: extern fn());
}

extern fn display_callback() {

}

fn test_manual_bindings() {
    unsafe {
        println!("FFI test. callings abs from c standard library: {}", abs(-3));
    }
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("Length: {}", x);

    let x = "test".as_bytes();
    let z = &(x.as_ptr());
    let y = z as *const *const u8;
    unsafe {
        glutInit(&0 as *const c_int, y);
        glutInitWindowSize(500, 500);
        glutInitWindowPosition(10, 10);
        glutCreateWindow(x.as_ptr());

        let err = glewInit();
        if err != GLEW_OK {
            let err_str = glewGetErrorString(err);
            let err_str = CStr::from_ptr(err_str);
            let str_slice: &str = err_str.to_str().unwrap();
            println!("Failed to start glew: {}", str_slice);
        }

        let version_str = glewGetString(GLEW_VERSION);
        let version_str = CStr::from_ptr(version_str);
        let str_slice: &str = version_str.to_str().unwrap();
        println!("Status: using GLEW version {}", str_slice);

        glutDisplayFunc(display_callback);
        glutMainLoop();
    }
}

fn main() {
    
}

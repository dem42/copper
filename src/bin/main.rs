#![allow(dead_code)]
extern crate libc;
use libc::{size_t, c_int, c_uint, c_char};
use std::str;
use std::ffi::CStr;

extern crate copper;

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


extern fn display_callback() {

}

fn test_manual_bindings() {
    unsafe {
        println!("FFI test. callings abs from c standard library: {}", abs(-3));
    }
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("Length: {}", x);

    let mut x = [0i8; 0];
    let z = &mut (x.as_mut_ptr());
    let y = z as *mut *mut c_char;    
    let mut argc = 0;
    unsafe {
        copper::glutInit(&mut argc as *mut c_int, y);
        copper::glutInitWindowSize(500, 500);
        copper::glutInitWindowPosition(10, 10);
        copper::glutCreateWindow(x.as_ptr());

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

        copper::glutDisplayFunc(Some(display_callback));
        copper::glutMainLoop();
    }
}

fn main() {
    test_manual_bindings();
}

#![allow(dead_code)]
extern crate libc;
use libc::{c_char, c_int, c_uint, size_t};
use std::ffi::CStr;
use std::str;

extern crate glew_sys;
extern crate glut_sys;

extern "C" fn display_callback() {}

fn test_manual_bindings() {
    let mut x = [0i8; 0];
    let z = &mut (x.as_mut_ptr());
    let y = z as *mut *mut c_char;
    let mut argc = 0;
    unsafe {
        glut_sys::glutInit(&mut argc as *mut c_int, y);
        glut_sys::glutInitWindowSize(500, 500);
        glut_sys::glutInitWindowPosition(10, 10);
        glut_sys::glutCreateWindow(x.as_ptr());

        let err = glew_sys::glewInit();
        if err != glew_sys::GLEW_OK {
            let err_str = glew_sys::glewGetErrorString(err) as *const i8;
            let err_str = CStr::from_ptr(err_str);
            let str_slice: &str = err_str.to_str().unwrap();
            println!("Failed to start glew: {}", str_slice);
        }

        let version_str = glew_sys::glewGetString(glew_sys::GLEW_VERSION) as *const i8;
        let version_str = CStr::from_ptr(version_str);
        let str_slice: &str = version_str.to_str().unwrap();
        println!("Status: using GLEW version {}", str_slice);

        glut_sys::glutDisplayFunc(Some(display_callback));
        glut_sys::glutMainLoop();
    }
}

fn main() {
    test_manual_bindings();
}

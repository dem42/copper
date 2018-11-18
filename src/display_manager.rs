use glfw_sys::*;
use glew_sys::*;
use libc::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::ptr;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const FPS_CAP: u32 = 120;

static mut window: *mut GLFWwindow = ptr::null_mut();

unsafe extern "C" fn error_callback(_error_code: c_int, description: *const c_char) {
    let err_str = description as *const i8;
    let err_str = CStr::from_ptr(err_str);
    let str_slice: &str = err_str.to_str().unwrap();
    println!("GLFW error: {}", str_slice);
}

pub fn create_display() {
    let title = CString::new("Hello Copper").expect("CString::new failed");    
    unsafe {        

        if glfwInit() != GLFW_TRUE as i32 {
            panic!("Unsuccessful initialziation of GLFW");
        }

        glfwSetErrorCallback(Some(error_callback));

        window = glfwCreateWindow(WIDTH, HEIGHT, title.into_raw(), ptr::null_mut(), ptr::null_mut());
        if window == ptr::null_mut() {
            glfwTerminate();
            panic!("Unable to create window and OpenGL context");
        }
    
        let err = glewInit();
        if err != GLEW_OK {
            let err_str = glewGetErrorString(err) as *const i8;
            let err_str = CStr::from_ptr(err_str);
            let str_slice: &str = err_str.to_str().unwrap();
            println!("Failed to start glew: {}", str_slice);
        }

        let version_str = glewGetString(GLEW_VERSION) as *const i8;
        let version_str = CStr::from_ptr(version_str);
        let str_slice: &str = version_str.to_str().unwrap();
        println!("Status: using GLEW version {}", str_slice);

        glfwMakeContextCurrent(window);
    }
}

pub fn update_display() {
    
}

pub fn close_display() {
    unsafe {
        glfwDestroyWindow(window);
        glfwTerminate();
    }
}

pub fn is_close_requested() -> bool {
    false
}
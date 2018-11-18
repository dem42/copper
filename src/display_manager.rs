use glfw_sys::*;
use glew_sys::{
    glewInit,    
    glewGetErrorString,
    glewGetString,
    GLEW_OK,
    GLEW_VERSION
};
use libc::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::ptr;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
// const FPS_CAP: u32 = 120;
// investigate framerate limits (high framerates vs VSync and flickering)
// what happens when we use lwjgl Display.Sync(frame_rate_cap)

static mut WINDOW: *mut GLFWwindow = ptr::null_mut();

unsafe extern "C" fn error_callback(_error_code: c_int, description: *const c_char) {
    let err_str = description as *const i8;
    let err_str = CStr::from_ptr(err_str);
    let str_slice: &str = err_str.to_str().unwrap();
    println!("GLFW error: {}", str_slice);
}

unsafe extern "C" fn key_callback(window: *mut GLFWwindow, key: c_int, scancode: c_int, action: c_int, mods: c_int) {
        
}

pub fn create_display() {
    let title = CString::new("Hello Copper").expect("CString::new failed");    
    unsafe {        

        if glfwInit() != GLFW_TRUE as i32 {
            panic!("Unsuccessful initialziation of GLFW");
        }

        glfwSetErrorCallback(Some(error_callback));

        // uncomment these lines if on Apple OS X
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR as i32, 2);
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT as i32, GL_TRUE as i32);
        glfwWindowHint(GLFW_OPENGL_PROFILE as i32, GLFW_OPENGL_CORE_PROFILE as i32);

        WINDOW = glfwCreateWindow(WIDTH, HEIGHT, title.into_raw(), ptr::null_mut(), ptr::null_mut());
        if WINDOW == ptr::null_mut() {
            glfwTerminate();
            panic!("Unable to create window and OpenGL context");
        }
        glfwSetKeyCallback(WINDOW, Some(key_callback));

        glfwMakeContextCurrent(WINDOW);
    
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
    }
}

pub fn update_display() {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut width_ptr = &mut width as *mut c_int;
    let mut height_ptr = &mut height as *mut c_int;
    unsafe { 
        // set the viewport size (measured in pixels unlike the window size which is in screen coordinates)
        glfwGetFramebufferSize(WINDOW, width_ptr, height_ptr);
        let ratio = (width as f32) / (height as f32);
        glfw_sys::glViewport(0 as c_int, 0 as c_int, width, height);

        glfw_sys::glClear(GL_COLOR_BUFFER_BIT);

        glfwPollEvents(); 
    }
}

pub fn close_display() {
    unsafe {
        glfwDestroyWindow(WINDOW);
        glfwTerminate();
    }
}

pub fn is_close_requested() -> bool {    
    unsafe {
        let close_flag = glfwWindowShouldClose(WINDOW);
        close_flag != 0
    }
}
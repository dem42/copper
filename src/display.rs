
use glfw::{
    Action,
    Context,
    WindowEvent,
};
use std::time::{
    SystemTime,
};
use std::sync::mpsc::Receiver;
use std::fmt;
use crate::gl;

pub use glfw::Key;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
// const FPS_CAP: u32 = 120;
// investigate framerate limits (high framerates vs VSync and flickering)
// what happens when we use lwjgl Display.Sync(frame_rate_cap)


pub trait Keyboard {
    fn is_pressed(&self, key: Key) -> bool;
}

#[derive(Default)]
pub struct MousePosData {
    pub prev_x: f64,
    pub prev_y: f64,
    pub cur_x: f64,
    pub cur_y: f64,   
    pub cur_scroll: f64,    
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_middle_pressed: bool,
}

impl MousePosData {
    pub fn dx(&self) -> f64 {
        self.cur_x - self.prev_x
    }
    pub fn dy(&self) -> f64 {
        self.cur_y - self.prev_y
    }
    pub fn d_scroll(&self) -> f64 {
        self.cur_scroll
    }
    pub fn set_prev_to_cur(&mut self) {
        self.prev_x = self.cur_x;
        self.prev_y = self.cur_y;
        self.cur_scroll = 0.0;
    }
}

impl fmt::Display for MousePosData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Mouse pos is: ({},{}). It moved by dx={}, dy={}. The button press states are ({},{},{}). Scroll is {}", 
            self.cur_x, self.cur_y,
            self.dx(), self.dy(),
            self.is_left_pressed,
            self.is_middle_pressed,
            self.is_right_pressed,
            self.d_scroll())        
    }
}

pub struct Display {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    last_frame_sys_time: SystemTime,
    pub frame_time_sec: f32,
    pub mouse_pos: MousePosData,
}

impl Keyboard for Display {
    fn is_pressed(&self, key: Key) -> bool {
        match self.window.get_key(key) {
            glfw::Action::Press => true,
            _ => false,
        }
    }
}

impl Display {
    pub fn create() -> Display {        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "Hello Copper", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);

        Display::print_opengl_info(&window);

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        gl::helper::register_error_callback();

        Display {
            glfw,
            window,
            events,
            last_frame_sys_time: SystemTime::now(),
            frame_time_sec: 0.0,
            mouse_pos: MousePosData::default(),
        }
    }

    pub fn get_aspect_ration(&self) -> f32 {
        let (width, height) = self.window.get_framebuffer_size();        
        let aspect_ratio = (width as f32) / (height as f32);
        aspect_ratio
    }
 
    fn print_opengl_info(window: &glfw::Window) {
        let gl_version = window.get_context_version();    
        let is_core_profile = window.get_opengl_profile() == glfw::OpenGlProfileHint::Core as i32;
        let is_forward_compat = window.is_opengl_forward_compat();
        println!("{}", "*".repeat(10));
        println!("OpenGL version: {}", gl_version);    
        println!("Core profile: {}, Forward compatibility: {}", is_core_profile, is_forward_compat);    
        println!("{}", "*".repeat(10));
    }

    pub fn update_display(&mut self) {
        // set the viewport size (measured in pixels unlike the window size which is in screen coordinates)
        let (width, height) = self.window.get_framebuffer_size();        
        let _ratio = (width as f32) / (height as f32);
        
        gl::viewport(0, 0, width, height);

        self.window.swap_buffers();

        self.glfw.poll_events();

        self.update_frame_time_measurement();

        self.mouse_pos.set_prev_to_cur();

        for (_, event) in glfw::flush_messages(&self.events) {
            Display::handle_window_event(&mut self.mouse_pos, event);
        }    
    }

    pub fn is_close_requested(&self) -> bool {  
        self.window.should_close()
    }

    fn update_frame_time_measurement(&mut self) {
        let current_time = SystemTime::now();
        let elapsed = current_time.duration_since(self.last_frame_sys_time);
        self.frame_time_sec = match elapsed {
            Ok(elapsed) => (elapsed.as_secs() as f32 + elapsed.subsec_micros() as f32 / 1_000_000.0),
            Err(_) => self.frame_time_sec,
        };
        self.last_frame_sys_time = current_time;
    }

    fn handle_window_event(mouse_pos: &mut MousePosData, event: WindowEvent) {
        match event {
            WindowEvent::CursorPos(x, y) => {
                mouse_pos.prev_x = mouse_pos.cur_x;
                mouse_pos.prev_y = mouse_pos.cur_y;
                mouse_pos.cur_x = x;
                mouse_pos.cur_y = y;
            },
            WindowEvent::MouseButton(button, action, _) => {
                match (button, action) {
                    (glfw::MouseButtonLeft, Action::Press) => { mouse_pos.is_left_pressed = true; },
                    (glfw::MouseButtonLeft, Action::Release) => { mouse_pos.is_left_pressed = false; },
                    (glfw::MouseButtonRight, Action::Press) => { mouse_pos.is_right_pressed = true; },
                    (glfw::MouseButtonRight, Action::Release) => { mouse_pos.is_right_pressed = false; },
                    (glfw::MouseButtonMiddle, Action::Press) => { mouse_pos.is_middle_pressed = true; },
                    (glfw::MouseButtonMiddle, Action::Release) => { mouse_pos.is_middle_pressed = false; },
                    _ => {}
                }
            },
            WindowEvent::Scroll(_x_scroll, y_scroll) => {                
                mouse_pos.cur_scroll = y_scroll;
            },
            _ => {}
        }
    }
}


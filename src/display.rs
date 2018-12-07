
use glfw::Context;
use super::gl;

pub use glfw::Key;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
// const FPS_CAP: u32 = 120;
// investigate framerate limits (high framerates vs VSync and flickering)
// what happens when we use lwjgl Display.Sync(frame_rate_cap)


pub trait Keyboard {
    fn is_pressed(&self, key: Key) -> bool;
}

pub struct Display {
    glfw: glfw::Glfw,
    window: glfw::Window,
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

        let (mut window, _events) = glfw.create_window(WIDTH, HEIGHT, "Hello Copper", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();

        Display::print_opengl_info(&window);

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        gl::helper::register_error_callback();

        Display {
            glfw,
            window
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

        // for (_, event) in glfw::flush_messages(&events) {
        //     handle_window_event(&mut window, event);
        // }    
    }

    pub fn is_close_requested(&self) -> bool {  
        self.window.should_close()
    }

    // fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    //     match event {
    //         glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
    //             window.set_should_close(true)
    //         }
    //         _ => {}
    //     }
    // }
}


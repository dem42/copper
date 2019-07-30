use crate::display::{
    Display,
};
use crate::gl;

pub struct Framebuffers {
    // water framebuffers
    pub reflection_fbo: ReflectionFBO,
    pub refraction_fbo: RefractionFBO,
    pub shadowmap_fbo: ShadowMapFBO,
}

pub struct ReflectionFBO {
    fbo_id: u32,
    pub color_texture: u32,
    depth_renderbuffer: u32,
}

pub struct RefractionFBO {
    fbo_id: u32,
    pub color_texture: u32,
    pub depth_texture: u32,
}

pub struct ShadowMapFBO {
    fbo_id: u32,
    pub depth_texture: u32,

}

impl Framebuffers {
    pub fn new(display: &Display) -> Self {
        let reflection_fbo = ReflectionFBO::new();
        let refraction_fbo = RefractionFBO::new();
        let shadowmap_fbo = ShadowMapFBO::new();
        display.restore_default_framebuffer();
        Framebuffers {
            reflection_fbo,
            refraction_fbo,
            shadowmap_fbo,
        }
    }
}

pub trait FramebufferObject {

    fn create_frame_buffer(has_color_buf: bool) -> u32 {
        let fbo_id = gl::gen_framebuffer();
        gl::bind_framebuffer(gl::FRAMEBUFFER, fbo_id);
        let color_attachments = if has_color_buf {        
            [gl::COLOR_ATTACHMENT0; 1]
        } else {
            [gl::NONE; 1]
        };
        gl::draw_buffers(&color_attachments);
        fbo_id
    }

    fn create_color_texture_attachment(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::RGB, gl::RGB, width, height, gl::UNSIGNED_BYTE);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, tex_id, 0);
        tex_id
    }

    fn create_depth_texture_attachment(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT, width, height, gl::FLOAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, tex_id, 0);
        tex_id
    }

    fn create_depth_texture_attachment_for_shadows(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT16, width, height, gl::FLOAT);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, tex_id, 0);
        tex_id
    }

    fn create_depth_renderbuffer_attachment(width: usize, height: usize) -> u32 {
        let render_buffer_id = gl::gen_renderbuffer();
        gl::bind_renderbuffer(gl::RENDERBUFFER, render_buffer_id);
        gl::renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
        gl::framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, render_buffer_id);
        render_buffer_id
    }

    fn bind_framebuffer(fbo_id: u32, width: usize, height: usize) {
        gl::bind_texture(gl::TEXTURE_2D, 0); // unbind any active texture
        gl::bind_framebuffer(gl::FRAMEBUFFER, fbo_id); // bind the frame buffer 
        gl::viewport(0, 0, width as i32, height as i32);
    }

    fn bind(&mut self);
}

impl ReflectionFBO {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    // const WIDTH: usize = 320;
    // const HEIGHT: usize = 180;

    fn new() -> Self {
        let fbo_id = Self::create_frame_buffer(true);
        let color_texture = Self::create_color_texture_attachment(ReflectionFBO::WIDTH, ReflectionFBO::HEIGHT);
        let depth_renderbuffer = Self::create_depth_renderbuffer_attachment(ReflectionFBO::WIDTH, ReflectionFBO::HEIGHT);
        ReflectionFBO {
            fbo_id,
            color_texture,
            depth_renderbuffer,
        }
    }
}

impl FramebufferObject for ReflectionFBO {
    fn bind(&mut self) {
        Self::bind_framebuffer(self.fbo_id, ReflectionFBO::WIDTH, ReflectionFBO::HEIGHT);
    }
}

impl Drop for ReflectionFBO {
    fn drop(&mut self) {
        gl::delete_framebuffer(self.fbo_id);
        gl::delete_texture(self.color_texture);
        gl::delete_renderbuffer(self.depth_renderbuffer);
    }    
}

impl RefractionFBO {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;

    fn new() -> Self {
        let fbo_id = Self::create_frame_buffer(true);
        let color_texture = Self::create_color_texture_attachment(RefractionFBO::WIDTH, RefractionFBO::HEIGHT);
        let depth_texture = Self::create_depth_texture_attachment(RefractionFBO::WIDTH, RefractionFBO::HEIGHT);
        RefractionFBO {
            fbo_id,
            color_texture,
            depth_texture,
        }
    }
}

impl FramebufferObject for RefractionFBO {
    fn bind(&mut self) {
        Self::bind_framebuffer(self.fbo_id, RefractionFBO::WIDTH, RefractionFBO::HEIGHT);
    }
}

impl Drop for RefractionFBO {
    fn drop(&mut self) {
        gl::delete_framebuffer(self.fbo_id);
        gl::delete_texture(self.color_texture);
        gl::delete_texture(self.depth_texture);        
    }    
}

impl ShadowMapFBO {
    const SHADOW_MAP_SIZE: usize = 2048;

    fn new() -> Self {
        let fbo_id = Self::create_frame_buffer(false);
        let depth_texture = Self::create_depth_texture_attachment_for_shadows(ShadowMapFBO::SHADOW_MAP_SIZE, ShadowMapFBO::SHADOW_MAP_SIZE);
        ShadowMapFBO {
            fbo_id,
            depth_texture,
        }
    }
}

impl FramebufferObject for ShadowMapFBO {
    fn bind(&mut self) {
        Self::bind_framebuffer(self.fbo_id, ShadowMapFBO::SHADOW_MAP_SIZE, ShadowMapFBO::SHADOW_MAP_SIZE);
    }
}
use super::super::Display;
use crate::gl;

bitflags! {
    pub struct FboFlags : u32 {
        const COLOR_TEX       = 0b000001;
        const COLOR_RENDERBUF = 0b000010;
        const DEPTH_TEX       = 0b000100;
        const DEPTH_RENDERBUF = 0b001000;
        const SHADOW_DEPTH    = 0b010000;
        const MULTISAMPLED    = 0b100000;
    }
}

pub struct FramebufferObject {
    fbo_id: u32,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub color_texture: Option<u32>,
    pub depth_texture: Option<u32>,
    pub depth_renderbuffer_id: Option<u32>,
    pub color_renderbuffer_id: Option<u32>,
}

impl FramebufferObject {
    // number of samples used for multisampled anti aliasing MSAA
    const SAMPLE_NUM: usize = 4;

    pub fn new(viewport_width: usize, viewport_height: usize, flags: FboFlags) -> Self {
        let fbo_id = Self::create_frame_buffer(flags);
        let color_texture = if flags.contains(FboFlags::COLOR_TEX) {
            Some(Self::create_color_texture_attachment(viewport_width, viewport_height))
        } else {
            None
        };
        let depth_texture = if flags.contains(FboFlags::DEPTH_TEX) {
            Some(Self::create_depth_texture_attachment(viewport_width, viewport_height))
        } else if flags.contains(FboFlags::SHADOW_DEPTH) {
            // can this be simplified into just one depth attachment?
            Some(Self::create_depth_texture_attachment_for_shadows(viewport_width, viewport_height))
        } else {
            None
        };
        let depth_renderbuffer_id = if flags.contains(FboFlags::DEPTH_RENDERBUF) {
            Some(Self::create_depth_renderbuffer_attachment(viewport_width, viewport_height, flags.contains(FboFlags::MULTISAMPLED)))
        } else {
            None
        };
        let color_renderbuffer_id = if flags.contains(FboFlags::COLOR_RENDERBUF) {
            Some(Self::create_color_renderbuffer_attachment(viewport_width, viewport_height, flags.contains(FboFlags::MULTISAMPLED)))
        } else {
            None
        };
        Self::check_framebuffer();
        FramebufferObject {
            fbo_id,
            viewport_width,
            viewport_height,
            color_texture,
            depth_texture,
            depth_renderbuffer_id,
            color_renderbuffer_id,
        }
    }

    pub fn create_frame_buffer(flags: FboFlags) -> u32 {
        let fbo_id = gl::gen_framebuffer();
        gl::bind_framebuffer(gl::FRAMEBUFFER, fbo_id);
        let color_attachments = if flags.intersects(FboFlags::COLOR_TEX | FboFlags::COLOR_RENDERBUF) {        
            [gl::COLOR_ATTACHMENT0; 1]
        } else {
            // by setting the draw_buffers to none we effectively make this a depth only pass
            [gl::NONE; 1]
        };
        gl::draw_buffers(&color_attachments);
        fbo_id
    }

    pub fn create_color_texture_attachment(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::RGB, gl::RGB, width, height, gl::UNSIGNED_BYTE);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, tex_id, 0);
        tex_id
    }

    pub fn create_depth_texture_attachment(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT, width, height, gl::FLOAT);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        gl::tex_parameter_iv(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, tex_id, 0);
        tex_id
    }

    pub fn create_depth_texture_attachment_for_shadows(width: usize, height: usize) -> u32 {
        let tex_id = gl::gen_texture();
        gl::bind_texture(gl::TEXTURE_2D, tex_id);
        gl::tex_image_2d_uninitialized(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT32, width, height, gl::FLOAT);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        gl::tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        // attach mipmap level 0 of texture (tex_id -> unitialized) to the color attach0 of current framebuffer 
        gl::framebuffer_texture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, tex_id, 0);
        tex_id
    }

    pub fn create_depth_renderbuffer_attachment(width: usize, height: usize, multisampled: bool) -> u32 {
        let render_buffer_id = gl::gen_renderbuffer();
        gl::bind_renderbuffer(gl::RENDERBUFFER, render_buffer_id);
        if multisampled {
            gl::renderbuffer_storage_multisampled(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height, Self::SAMPLE_NUM);
        } else {
            gl::renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
        }
        gl::framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, render_buffer_id);
        render_buffer_id
    }

    pub fn create_color_renderbuffer_attachment(width: usize, height: usize, multisampled: bool) -> u32 {
        let render_buffer_id = gl::gen_renderbuffer();
        gl::bind_renderbuffer(gl::RENDERBUFFER, render_buffer_id);
        if multisampled {
            gl::renderbuffer_storage_multisampled(gl::RENDERBUFFER, gl::RGBA8, width, height, Self::SAMPLE_NUM);
        } else {
            gl::renderbuffer_storage(gl::RENDERBUFFER, gl::RGBA8, width, height);
        }
        gl::framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, render_buffer_id);
        render_buffer_id
    }

    pub fn bind_framebuffer(fbo_id: u32, width: usize, height: usize) {
        gl::bind_framebuffer(gl::FRAMEBUFFER, fbo_id); // bind the frame buffer 
        gl::viewport(0, 0, width as i32, height as i32);
    }

    pub fn check_framebuffer() {
        gl::check_framebuffer_status(gl::FRAMEBUFFER);
    }

    pub fn bind(&mut self) {
        Self::bind_framebuffer(self.fbo_id, self.viewport_width, self.viewport_height);
    }

    pub fn resolve_to_fbo(&mut self, target_fbo: &mut FramebufferObject, display: &Display) {
        // draw to target
        gl::bind_framebuffer(gl::DRAW_FRAMEBUFFER, target_fbo.fbo_id);
        // read from us
        gl::bind_framebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
        gl::blit_framebuffer(0, 0, self.viewport_width, self.viewport_height, 0, 0, target_fbo.viewport_width, target_fbo.viewport_height, gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT, gl::NEAREST);
        display.restore_default_framebuffer();
    }

    pub fn resolve_to_screen(&mut self, display: &Display) {
        // draw to default fbo
        gl::bind_framebuffer(gl::DRAW_FRAMEBUFFER, 0);
        // read from us
        gl::bind_framebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
        let size = display.get_size();
        let width = size.0 as usize;
        let height = size.1 as usize;
        gl::blit_framebuffer(0, 0, self.viewport_width, self.viewport_height, 0, 0, width, height, gl::COLOR_BUFFER_BIT, gl::NEAREST);
        display.restore_default_framebuffer();
    }
}

impl Drop for FramebufferObject {
    fn drop(&mut self) {
        gl::delete_framebuffer(self.fbo_id);
        if let Some(color_tex) = self.color_texture {
            gl::delete_texture(color_tex);
        }
        if let Some(depth_tex) = self.depth_texture {
            gl::delete_texture(depth_tex);
        }
        if let Some(depth_renderbuf) = self.depth_renderbuffer_id {
            gl::delete_renderbuffer(depth_renderbuf);
        }
        if let Some(color_renderbuf) = self.color_renderbuffer_id {
            gl::delete_renderbuffer(color_renderbuf);
        }
    }    
}
use crate::gl;

bitflags! {
    pub struct FboFlags : u32 {
        const COLOR_TEX       = 0b00001;
        const COLOR_RENDERBUF = 0b00010;
        const DEPTH_TEX       = 0b00100;
        const DEPTH_RENDERBUF = 0b01000;
        const SHADOW_DEPTH    = 0b10000;
    }
}

pub struct FramebufferObject {
    fbo_id: u32,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub color_texture: Option<u32>,
    pub depth_texture: Option<u32>,
    pub depth_renderbuffer_id: Option<u32>,
}

impl FramebufferObject {

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
            Some(Self::create_depth_renderbuffer_attachment(viewport_width, viewport_height))
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
        }
    }

    pub fn create_frame_buffer(flags: FboFlags) -> u32 {
        let fbo_id = gl::gen_framebuffer();
        gl::bind_framebuffer(gl::FRAMEBUFFER, fbo_id);
        let color_attachments = if flags.contains(FboFlags::COLOR_TEX) {        
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

    pub fn create_depth_renderbuffer_attachment(width: usize, height: usize) -> u32 {
        let render_buffer_id = gl::gen_renderbuffer();
        gl::bind_renderbuffer(gl::RENDERBUFFER, render_buffer_id);
        gl::renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
        gl::framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, render_buffer_id);
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
    }    
}
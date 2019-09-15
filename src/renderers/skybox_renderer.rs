use crate::display::{
    WallClock,
};
use crate::entities::{
    Camera,
    Skybox,
};
use crate::gl;
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,
};
use crate::models::{
    RawModel,
};
use crate::shaders::SkyboxShader;

pub struct SkyboxRenderer {
    shader: SkyboxShader,    
}

impl SkyboxRenderer {
    pub fn new(proj_matrix: &Matrix4f) -> SkyboxRenderer {
        let mut skybox_shader = SkyboxShader::new();
        skybox_shader.start();
        skybox_shader.load_projection_matrix(proj_matrix);
        skybox_shader.stop();
        SkyboxRenderer {
            shader: skybox_shader,
        }
    }

    pub fn render(&mut self, camera: &Camera, skybox: &Skybox, sky_color: &Vector3f, wall_clock: &WallClock, clip_plane: &Vector4f) {
        if skybox.invisible {
            return;
        }
        self.shader.start();        
        self.shader.load_view_matrix(camera, skybox.rotation_yaw_deg);
        self.shader.load_sky_color(sky_color, skybox.uses_fog); // due to day night this color needs to be set every frame
        // water stuff (every frame?)
        self.shader.load_clip_plane(clip_plane);
        
        self.bind_textures(skybox, wall_clock);        

        gl::bind_vertex_array(skybox.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::draw_arrays(gl::TRIANGLES, 0, skybox.model.raw_model.vertex_count);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);

        gl::bind_texture(gl::TEXTURE_CUBE_MAP, 0);

        self.shader.stop();
    }

    fn bind_textures(&mut self, skybox: &Skybox, wall_clock: &WallClock) {
        let (day_tex, night_tex, blend_factor) = skybox.get_day_night_textures(wall_clock);
        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_CUBE_MAP, day_tex.unwrap());
        if skybox.model.cycles_day_night {
            gl::active_texture(gl::TEXTURE1);
            gl::bind_texture(gl::TEXTURE_CUBE_MAP, night_tex.unwrap());
        }

        self.shader.load_blend_factor(blend_factor);
        self.shader.connect_texture_units();   
    }
}
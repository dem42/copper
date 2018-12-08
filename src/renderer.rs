use super::gl;
use super::entities::Entity;
use super::shaders::StaticShader;
use super::math::Matrix4f;
use super::display::Display;

pub struct Renderer {
    projection_matrix: Matrix4f,
}

impl Renderer {
    const FOV_HORIZONTAL: f32 = 70.0;
    // here using actual world coords which are RHS coord sys with z axis going into screen (so more negative means further)
    const NEAR: f32 = -0.1;
    const FAR: f32 = -1000.0;
    
    pub fn new(display: &Display, shader: &mut StaticShader) -> Renderer {
        let projection_matrix = Matrix4f::create_projection_matrix(Renderer::NEAR, Renderer::FAR, Renderer::FOV_HORIZONTAL, display.get_aspect_ration());
        shader.start();
        shader.load_projection_matrix(&projection_matrix);
        shader.stop();
        Renderer {
            projection_matrix
        }
    }

    pub fn prepare(&self) {
        gl::enable(gl::DEPTH_TEST);
        gl::clear_color((1.0, 0.0, 0.0, 1.0));
        gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    pub fn render(&self, entity: &Entity, shader: &mut StaticShader) {
        gl::bind_vertex_array(entity.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(entity.model.raw_model.pos_attrib);
        gl::enable_vertex_attrib_array(entity.model.raw_model.tex_coord_attrib);
        gl::enable_vertex_attrib_array(entity.model.raw_model.normal_attrib);

        // load transform matrix into shader
        let transform_mat = Matrix4f::create_transform_matrix(&entity.position, &entity.rotation_deg, entity.scale);
        shader.load_transformation_matrix(&transform_mat);
        shader.load_shine_variables(entity.model.texture.shine_damper, entity.model.texture.reflectivity);

        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(entity.model.texture.tex_id, gl::TEXTURE_2D);
        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);

        gl::disable_vertex_attrib_array(entity.model.raw_model.pos_attrib);
        gl::disable_vertex_attrib_array(entity.model.raw_model.tex_coord_attrib);
        gl::disable_vertex_attrib_array(entity.model.raw_model.normal_attrib);

        gl::bind_vertex_array(0);
        gl::bind_texture(0, gl::TEXTURE_2D);
    }
}
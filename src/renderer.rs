use super::loader::TexturedModel;
use super::gl;

pub struct Renderer;

impl Renderer {    
    pub fn new() -> Renderer {
        Renderer
    }

    pub fn prepare(&self) {
        gl::clear_color((1.0, 0.0, 0.0, 1.0));
        gl::clear(gl::COLOR_BUFFER_BIT);
    }

    pub fn render(&self, model: &TexturedModel) {
        gl::bind_vertex_array(model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(model.raw_model.pos_attrib);
        gl::enable_vertex_attrib_array(model.raw_model.tex_coord_attrib);
        gl::active_texture(gl::TEXTURE0); // activate bank 0
        gl::bind_texture(model.texture.tex_id, gl::TEXTURE_2D);
        gl::draw_elements(gl::TRIANGLES, model.raw_model.vertex_count, gl::UNSIGNED_INT);
        gl::disable_vertex_attrib_array(model.raw_model.pos_attrib);
        gl::disable_vertex_attrib_array(model.raw_model.tex_coord_attrib);
        gl::bind_vertex_array(0);
        gl::bind_texture(0, gl::TEXTURE_2D);
    }
}
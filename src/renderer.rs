use super::loader::{RawModel};
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

    pub fn render(&self, model: &RawModel) {
        gl::bind_vertex_array(model.vao_id);
        gl::enable_vertex_attrib_array(model.attribute_id);
        // gl::draw_arrays(gl::TRIANGLES, 0, model.vertex_count); when not using index buffer
        gl::draw_elements(gl::TRIANGLES, model.vertex_count, gl::UNSIGNED_INT);
        gl::disable_vertex_attrib_array(model.attribute_id);
        gl::bind_vertex_array(0);
    }
}
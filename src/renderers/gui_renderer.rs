use crate::guis::{
    Gui,
};
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
};
use crate::gl;
use crate::shaders::GuiShader;

pub struct GuiRenderer {
    shader: GuiShader,
}

impl GuiRenderer {
    pub fn new() -> GuiRenderer {
        GuiRenderer {
            shader: GuiShader::new(),
        }
    }

    pub fn render(&mut self, guis: &Vec<Gui>, gui_model: &RawModel) {
        self.shader.start();
        gl::bind_vertex_array(gui_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);

        // turn on alpha blending
        gl::enable(gl::BLEND);
        // linear blending
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        // with guis we want to be able to draw one gui on top of another even tho both have z = 0, so we disable depth test
        gl::disable(gl::DEPTH_TEST);

        for gui in guis.iter() {
            gl::active_texture(gl::TEXTURE0);
            gl::bind_texture(gui.texture_id, gl::TEXTURE_2D);
            let transform_mat = Matrix4f::create_gui_transform_matrix(&gui.position, &gui.scale);
            self.shader.load_transformation_matrix(&transform_mat);
            gl::draw_arrays(gl::TRIANGLE_STRIP, 0, gui_model.vertex_count);
        }

        gl::enable(gl::DEPTH_TEST);
        gl::disable(gl::BLEND);
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);
        self.shader.stop();
    }
}
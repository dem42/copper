use crate::guis::{
    GuiPanel,
    GuiText,
};
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
};
use crate::gl;
use crate::shaders::{
    GuiShader,
    TextShader,
};

pub struct GuiRenderer {
    gui_shader: GuiShader,
    text_shader: TextShader,
}

impl GuiRenderer {
    pub fn new() -> GuiRenderer {
        GuiRenderer {
            gui_shader: GuiShader::new(),
            text_shader: TextShader::new(),
        }
    }

    pub fn render(&mut self, guis: &Vec<GuiPanel>, gui_model: &RawModel, texts: &Vec<GuiText>) {

        // turn on alpha blending
        gl::enable(gl::BLEND);
        // linear blending
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        self.gui_shader.start();
        gl::bind_vertex_array(gui_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);

        // with guis we want to be able to draw one gui on top of another even tho both have z = 0, so we disable depth test
        gl::disable(gl::DEPTH_TEST);

        for gui in guis.iter() {
            gl::active_texture(gl::TEXTURE0);
            gl::bind_texture(gl::TEXTURE_2D, gui.texture_id);
            let transform_mat = Matrix4f::create_gui_transform_matrix(&gui.position, &gui.scale);
            self.gui_shader.load_transformation_matrix(&transform_mat);
            gl::draw_arrays(gl::TRIANGLE_STRIP, 0, gui_model.vertex_count);
        }

        
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);
        self.gui_shader.stop();


        self.text_shader.start();        
        for text in texts.iter() {
            gl::bind_vertex_array(text.text_mesh_vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);

            gl::active_texture(gl::TEXTURE0);
            gl::bind_texture(gl::TEXTURE_2D, text.font_type.texture_atlas);
            gl::draw_arrays(gl::TRIANGLES, 0, text.text_char_count * 4);
        }
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::bind_vertex_array(0);
        self.text_shader.stop();

        gl::enable(gl::DEPTH_TEST);
        gl::disable(gl::BLEND);
        gl::bind_texture(gl::TEXTURE_2D, 0);
    }
}
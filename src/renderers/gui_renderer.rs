use std::collections::HashMap;
use crate::guis::{
    GuiPanel,
    GuiText,
    text::FontType,
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
use super::master_renderer::RenderGroup;

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
        gl::helper::push_debug_group(RenderGroup::DRAW_GUI.id, RenderGroup::DRAW_GUI.name);
        
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
        let text_by_font = GuiRenderer::group_text_by_font(texts);
        for (font_type, text_vec) in text_by_font.iter() {
            gl::active_texture(gl::TEXTURE0);
            gl::bind_texture(gl::TEXTURE_2D, font_type.texture_atlas.unwrap());

            for text in text_vec.iter() {
                self.text_shader.load_position(&text.position);
                self.text_shader.load_text_material(&text.material);

                gl::bind_vertex_array(text.text_model.vao_id);
                gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);
                gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);

                gl::draw_arrays(gl::TRIANGLES, 0, text.text_model.vertex_count);
            }
        }
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::bind_vertex_array(0);
        self.text_shader.stop();

        gl::enable(gl::DEPTH_TEST);
        gl::disable(gl::BLEND);
        gl::bind_texture(gl::TEXTURE_2D, 0);

        gl::helper::pop_debug_group();
    }

    fn group_text_by_font(texts: &Vec<GuiText>) -> HashMap<&FontType, Vec<&GuiText>> {
        let mut result = HashMap::new();
        for text in texts.iter() {
            let group = result.entry(&text.font_type).or_insert(Vec::new());
            group.push(text);
        }
        result
    }
}
use crate::entities::{
    Camera,
    WaterTile,
};
use crate::gl;
use crate::models::{
    RawModel,
};
use crate::math::{
    Matrix4f,
};
use crate::shaders::WaterShader;

pub struct WaterRenderer {
    shader: WaterShader,
}

impl WaterRenderer {
    pub fn new(projection_mat: &Matrix4f) -> Self {
        let mut shader = WaterShader::new();
        shader.start();
        shader.load_projection_matrix(projection_mat);
        shader.stop();        
        WaterRenderer {
            shader,
        }
    }

    pub fn render(&mut self, water_tiles: &Vec<WaterTile>, camera: &Camera) {
        self.shader.start();
        let view_matrix = Matrix4f::create_view_matrix(camera);
        self.shader.load_view_matrix(&view_matrix);

        for water_tile in water_tiles {
            let transform_matrix = &water_tile.transform;
            self.shader.load_transform_matrix(transform_matrix);

            gl::bind_vertex_array(water_tile.model.vao_id);
            gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);            
            gl::draw_arrays(gl::TRIANGLE_STRIP, 0, water_tile.model.vertex_count);
            gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
            gl::bind_vertex_array(0);
        }

        self.shader.stop();
    }
}
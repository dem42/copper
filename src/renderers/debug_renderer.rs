use crate::gl;
use crate::entities::{
    DebugEntity,
    Camera,    
};
use crate::shaders::DebugShader;
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::models::{    
    RawModel,
    DynamicVertexIndexedModel,
};

pub struct DebugRenderer {
    shader: DebugShader,
    mvp_matrix: Matrix4f,
    proj_matrix: Matrix4f,
    view_matrix: Matrix4f,
}

impl DebugRenderer {    

    const CUBE_VERTS: [Vector3f; 8] = [
        Vector3f {x: -0.5, y: -0.5, z: 0.5},
        Vector3f {x: 0.5, y: -0.5, z: 0.5},
        Vector3f {x: 0.5, y: 0.5, z: 0.5},
        Vector3f {x: -0.5, y: 0.5, z: 0.5},
        Vector3f {x: -0.5, y: -0.5, z: -0.5},
        Vector3f {x: 0.5, y: -0.5, z: -0.5},
        Vector3f {x: 0.5, y: 0.5, z: -0.5},
        Vector3f {x: -0.5, y: 0.5, z: -0.5},
    ];
    
    pub fn new(projection_matrix: &Matrix4f) -> Self {     
        let shader = DebugShader::new();
        let mut proj_matrix = Matrix4f::identity();
        proj_matrix.post_multiply_in_place(projection_matrix);
        let view_matrix = Matrix4f::identity();
        let mvp_matrix = Matrix4f::identity();
        DebugRenderer {
            shader,
            mvp_matrix,
            proj_matrix,
            view_matrix,
        }
    }
    
    pub fn render_cube(&mut self, entity: &DebugEntity, camera: &Camera, ) {
        self.render(entity, camera, &Self::CUBE_VERTS);
    }

    pub fn render(&mut self, entity: &DebugEntity, camera: &Camera, vertices: &[Vector3f; 8]) {
        self.shader.start();
        self.view_matrix = Matrix4f::create_view_matrix(camera);

        // turn on alpha blending
        gl::enable(gl::BLEND);
        // linear blending
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::helper::disable_culling();
        
        gl::bind_vertex_array(entity.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);

        // load transform matrix into shader        
        self.mvp_matrix.make_identity();
        // dont use rotation for the moment
        let transform = Matrix4f::create_transform_matrix(&entity.position, &Vector3f::ZERO, 1.0);
        self.mvp_matrix.pre_multiply_in_place(&transform);
        self.mvp_matrix.pre_multiply_in_place(&self.view_matrix);        
        self.mvp_matrix.pre_multiply_in_place(&self.proj_matrix);
        self.shader.load_mvp_matrix(&self.mvp_matrix);

        self.update_vbo(&entity.model, vertices);

        gl::draw_elements(gl::TRIANGLES, entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);

        gl::disable(gl::BLEND);
        gl::helper::enable_backface_culling();
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);

        self.shader.stop();
    }

    fn update_vbo(&mut self, model: &DynamicVertexIndexedModel, vertices: &[Vector3f; 8]) {
        let vbo = model.stream_draw_vbo;
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
        gl::buffer_data_unitialized::<f32>(gl::ARRAY_BUFFER, 8 * 3, gl::STREAM_DRAW);
        let mut data: [f32; 8 * 3] = Default::default();
        for i in 0..8 {
            data[3*i] = vertices[i].x;
            data[3*i + 1] = vertices[i].y;
            data[3*i + 2] = vertices[i].z;
        }
        gl::buffer_sub_data(gl::ARRAY_BUFFER, 0, &data);
        gl::bind_buffer(gl::ARRAY_BUFFER, 0);
    }
}
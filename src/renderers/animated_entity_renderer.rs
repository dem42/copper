use crate::animations::AnimatedModel;
use crate::gl;
use crate::entities::{
    AnimatedEntity,
    Camera,    
};
use crate::shaders::AnimatedModelShader;
use crate::math::{
    Matrix4f,
    Vector3f,
};
use crate::models::{    
    RawModel,    
};

pub struct AnimatedEntityRenderer {
    shader: AnimatedModelShader,
    mvp_matrix: Matrix4f,
    proj_matrix: Matrix4f,
    view_matrix: Matrix4f,
}

impl AnimatedEntityRenderer {
    
    pub fn new(projection_matrix: &Matrix4f) -> Self {     
        let shader = AnimatedModelShader::new();
        let mut proj_matrix = Matrix4f::identity();
        proj_matrix.post_multiply_in_place(projection_matrix);
        let view_matrix = Matrix4f::identity();
        let mvp_matrix = Matrix4f::identity();
        Self {
            shader,
            mvp_matrix,
            proj_matrix,
            view_matrix,
        }
    }
    
    pub fn render_entities(&mut self, entities: &Vec<AnimatedEntity>, camera: &Camera) {
        for entity in entities {
            self.render(entity, camera);
        }
    }

    pub fn render(&mut self, animated_entity: &AnimatedEntity, camera: &Camera) {
        self.shader.start();
        self.view_matrix = Matrix4f::create_view_matrix(camera);

        gl::active_texture(gl::TEXTURE0);
        gl::bind_texture(gl::TEXTURE_2D, animated_entity.model.tex_id.unwrap());

        gl::bind_vertex_array(animated_entity.model.raw_model.vao_id);
        gl::enable_vertex_attrib_array(RawModel::POS_ATTRIB);

        // load transform matrix into shader        
        self.mvp_matrix.make_identity();
        // dont use rotation for the moment
        let transform = Matrix4f::create_transform_matrix(&animated_entity.position, &Vector3f::ZERO, animated_entity.scale);
        self.mvp_matrix.pre_multiply_in_place(&transform);
        self.mvp_matrix.pre_multiply_in_place(&self.view_matrix);        
        self.mvp_matrix.pre_multiply_in_place(&self.proj_matrix);
        self.shader.load_mvp_matrix(&self.mvp_matrix);

        gl::draw_elements(gl::TRIANGLES, animated_entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);
        
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::bind_vertex_array(0);

        self.shader.stop();
    }
}
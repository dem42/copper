use crate::animations::joint::AccumulatedJointTransforms;
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
    accumulator: AccumulatedJointTransforms,
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
            accumulator: AccumulatedJointTransforms::new(),
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
        gl::enable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::JOINT_IDX_ATTRIB);
        gl::enable_vertex_attrib_array(RawModel::JOINT_WEIGHT_ATTRIB);

        // load transform matrix into shader        
        self.mvp_matrix.make_identity();
        // dont use rotation for the moment
        let transform = Matrix4f::create_transform_matrix(&animated_entity.position, &animated_entity.rotation_deg, animated_entity.scale);
        self.mvp_matrix.pre_multiply_in_place(&transform);
        self.mvp_matrix.pre_multiply_in_place(&self.view_matrix);        
        self.mvp_matrix.pre_multiply_in_place(&self.proj_matrix);
        self.shader.load_mvp_matrix(&self.mvp_matrix);

        animated_entity.model.root_joint.collect_transforms(&mut self.accumulator);
        self.shader.load_joint_transforms(&self.accumulator);

        gl::draw_elements(gl::TRIANGLES, animated_entity.model.raw_model.vertex_count, gl::UNSIGNED_INT);
        
        gl::disable_vertex_attrib_array(RawModel::POS_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::TEX_COORD_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::NORMAL_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::JOINT_IDX_ATTRIB);
        gl::disable_vertex_attrib_array(RawModel::JOINT_WEIGHT_ATTRIB);
        gl::bind_vertex_array(0);

        self.shader.stop();
    }
}
use super::shader_program::ShaderProgram;
use crate::models::RawModel;
use crate::math::{
    Matrix4f,
    Vector3f,
};

const MAX_JOINTS: usize = 50;

pub struct AnimatedModelShader {
    shader_program: ShaderProgram,
    location_mvp_matrix: i32,
    location_light_direction: i32,
    location_diffuse_map: i32,
    location_joint_transforms: [i32; MAX_JOINTS],
}

impl AnimatedModelShader {

    pub fn new() -> Self {
        let (
            mut location_mvp_matrix,
            mut location_light_direction,
            mut location_diffuse_map,            
        ) = Default::default();
        let mut location_joint_transforms = [0i32; MAX_JOINTS];
        
        let shader_program = ShaderProgram::new(
            "res/shaders/animations/animModelVert.glsl",
            None,
            "res/shaders/animations/animModelFrag.glsl",
            |shader_prog| {
                shader_prog.bind_attribute(RawModel::POS_ATTRIB, "in_position");
                shader_prog.bind_attribute(RawModel::TEX_COORD_ATTRIB, "in_tex_coords");
                shader_prog.bind_attribute(RawModel::NORMAL_ATTRIB, "in_normal");
                shader_prog.bind_attribute(RawModel::JOINT_IDX_ATTRIB, "in_joint_indicies");
                shader_prog.bind_attribute(RawModel::JOINT_WEIGHT_ATTRIB, "in_joint_weights");
            },
            |shader_prog| {                
                location_diffuse_map = shader_prog.get_uniform_location("diffuse_map");
                location_mvp_matrix = shader_prog.get_uniform_location("projection_view_model");
                location_light_direction = shader_prog.get_uniform_location("light_direction");
                // diffuse lighting                
                for i in 0..MAX_JOINTS {
                    // TODO: maybe we should optimize these string allocations that we keep doing
                    location_joint_transforms[i] = shader_prog.get_uniform_location(&format!("joint_transforms[{}]", i));                    
                }                
        });
        AnimatedModelShader {
            shader_program,
            location_mvp_matrix,
            location_light_direction,
            location_diffuse_map,
            location_joint_transforms,
        }
    }

    pub fn start(&mut self) {
        self.shader_program.start();
    }

    pub fn stop(&mut self) {
        self.shader_program.stop();
    }

    pub fn load_joint_transforms(&mut self, joint_transforms: &Vec<&Matrix4f>) {
        let joint_cnt = usize::min(joint_transforms.len(), MAX_JOINTS);
        for i in 0..joint_cnt {
            ShaderProgram::load_matrix(self.location_joint_transforms[i], joint_transforms[i]);
        }
    }

    pub fn load_mvp_matrix(&mut self, mvp: &Matrix4f) {
        ShaderProgram::load_matrix(self.location_mvp_matrix, mvp);
    }

    pub fn load_light_direction(&mut self, light_position: &Vector3f) {
        ShaderProgram::load_vector3d(self.location_light_direction, light_position);
    }

    pub fn connect_texture_units(&mut self) {
        ShaderProgram::load_int(self.location_diffuse_map, 0);
    }
}
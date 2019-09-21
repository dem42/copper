use super::loader::{
    ExtraInfo,
    ModelLoader,
    RawModel,
    TextureParams,
};
use crate::animations::{
    animation::{
        Animation,
        JointAnimation,
    },
    keyframe::Keyframe,
    animated_model::AnimatedModel,    
    joint::{
        Joint,
        JointTransform,
    },
};
use crate::math::Matrix4f;
use collada::{
    Matrix4,
    document::ColladaDocument,
};
use std::hash::{Hash, Hasher};

struct VertexData {
    n_vidx: usize,
    vidx: usize,
    tidx: usize,
    nidx: usize,
}

impl PartialEq for VertexData {
    fn eq(&self, other: &VertexData) -> bool {
        self.vidx == other.vidx 
            && self.tidx == self.tidx
            && self.nidx == self.nidx
    }
}

impl Eq for VertexData {}

impl Hash for VertexData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vidx.hash(state);        
        self.tidx.hash(state);        
        self.nidx.hash(state);        
    }
}

pub fn load_collada_animation(loader: &mut ModelLoader, path: &str, texture_path: &str) -> (AnimatedModel, Animation) {
    let path = std::path::Path::new(path);
    let collada_doc = ColladaDocument::from_path(&path).expect(&format!("Failed to load collada document: {:?}", path));

    let animation = animations_from_collada(&collada_doc);

    let animated_raw_model = raw_model_from_obj_set(&collada_doc, loader);
    let texture_id = loader.load_texture_internal(texture_path, TextureParams::default(), ExtraInfo::default());

    let bind_data_set = collada_doc.get_bind_data_set().expect("Collada file must contain bind data");
    
    let root_joint = joints_from_collada(&collada_doc);
    let joint_cnt = root_joint.children.len() + 1;
    (
        AnimatedModel {
            raw_model: animated_raw_model,
            tex_id: texture_id,
            root_joint,
            joint_cnt,
        },
        animation
    )
}

fn joints_from_collada(collada_doc: &ColladaDocument) -> Joint {
    let mut joint_vec = Vec::new();
     
    let skeletons = collada_doc.get_skeletons().expect("Collada file must contain skeleton");
    assert!(skeletons.len() == 1, "We support only one skeleton in the collada file");
    let skeleton = &skeletons[0];

    let mut root_idx = 0;
    let mut cnt = 0;
    for bone in skeleton.joints.iter() {
        let joint = Joint::new(cnt, bone.parent_index as usize, bone.name.clone(), convert_to_row_mat(&bone.inverse_bind_pose));
        if bone.is_root() {
            root_idx = cnt;
        }    
        joint_vec.push(joint);
        cnt += 1;
    }
    for joint in joint_vec.iter_mut() {

    }
    joint_vec.remove(root_idx)
}

fn convert_to_row_mat(col_mjr_mat: &Matrix4<f32>) -> Matrix4f {
    let mut res = Matrix4f::zeros();
    for i in 0..4 {
        for j in 0..4 {
            res[i][j] = col_mjr_mat[j][i];
        }
    }
    res
}

fn animations_from_collada(collada_doc: &ColladaDocument) -> Animation {
    let animations = collada_doc.get_animations().expect("Collada file must contain animations");

    let mut animation = Animation {
        length_seconds: 0.0,
        joint_animations: Vec::new(),
    };
    for a in animations {
        let mut keyframes = Vec::new();
        let mut length_seconds = 0.0;
        for i in 0..a.sample_poses.len() {
            length_seconds += a.sample_times[i];
            keyframes.push(
                Keyframe {
                    timestamp: a.sample_times[i],
                    pose: JointTransform::create_from_collada(&a.sample_poses[i]), 
                }
            );
        }
        animation.joint_animations.push(
            JointAnimation {
                name: a.target,
                length_seconds,
                keyframes,
            }
        );
    }
    animation
}

fn raw_model_from_obj_set(collada_doc: &ColladaDocument, loader: &mut ModelLoader) -> RawModel {
    let obj_set = collada_doc.get_obj_set().expect("Collada file must contain objects").objects;
    
    assert!(obj_set.len() == 1, "At the moment we support only having one animated object per collada file");
    let obj_set = &obj_set[0];
    
    let mut vertex_data = std::collections::HashSet::<VertexData>::new();
    let mut vertices_seen = std::collections::HashSet::<usize>::new();
    
    let mut indices = Vec::new();
                        
    assert!(obj_set.geometry.len() == 1, "Only support triangle mesh");
    let geometry = &obj_set.geometry[0];
    let mut nvidx_gen = obj_set.vertices.len();
    for geo in geometry.mesh.iter() {
        match geo {
            collada::PrimitiveElement::Triangles(triangles) => {
                let num = triangles.vertices.len();
                let vertices = &triangles.vertices;
                let tex_vertices = triangles.tex_vertices.as_ref().expect("Must have tex coords for mesh in collada file");
                let normals = triangles.normals.as_ref().expect("Must have normals for mesh in collada file");
                for i in 0..num {
                    let vs = [vertices[i].0, vertices[i].1, vertices[i].2];
                    let ts = [tex_vertices[i].0, tex_vertices[i].1, tex_vertices[i].2];
                    let ns = [normals[i].0, normals[i].1, normals[i].2];
                    for j in 0..3 {
                        let mut nvert_data = VertexData {
                            n_vidx: vs[j],
                            vidx: vs[j],
                            tidx: ts[j],
                            nidx: ns[j],
                        };
                        if !vertex_data.contains(&nvert_data) {
                            if vertices_seen.contains(&nvert_data.vidx) {
                                nvert_data.n_vidx = nvidx_gen;
                                nvidx_gen += 1;
                                println!("Duplicated vertex found -- this is a vertex where for this vertex position we have multiple different (v,t,n) tuples");
                            } else {
                                vertices_seen.insert(nvert_data.vidx);
                            } 
                            indices.push(nvert_data.n_vidx as u32);
                            vertex_data.insert(nvert_data);
                        } else {
                            let vidx = vertex_data.get(&nvert_data).unwrap().n_vidx;
                            indices.push(vidx as u32);
                        }
                    }
                }
            },
            _ => panic!("Unsupported geometry"),
        }
    }

    let mut positions = vec![0f32; nvidx_gen * 3];
    let mut texture_coords = vec![0f32; nvidx_gen * 2];;
    let mut normals = vec![0f32; nvidx_gen * 3];
    let mut joint_weights = vec![0f32; nvidx_gen * 4];
    let mut joint_indices = vec![0i32; nvidx_gen * 4];
    let mut vertex_data = vertex_data.iter().collect::<Vec<_>>();
    vertex_data.sort_by(|a, b| a.n_vidx.cmp(&b.n_vidx));

    let mut idx = 0;
    for v_data in vertex_data {
        positions[3*idx] = obj_set.vertices[v_data.vidx].x as f32;
        positions[3*idx + 1] = obj_set.vertices[v_data.vidx].y as f32;
        positions[3*idx + 2] = obj_set.vertices[v_data.vidx].z as f32;

        for i in 0..4 {
            joint_weights[4*idx + i] = obj_set.joint_weights[v_data.vidx].weights[i] as f32;
            joint_indices[4*idx + i] = obj_set.joint_weights[v_data.vidx].joints[i] as i32;
        }
        
        texture_coords[2*idx] = obj_set.tex_vertices[v_data.tidx].x as f32;
        texture_coords[2*idx + 1] = obj_set.tex_vertices[v_data.tidx].y as f32;

        normals[3*idx] = obj_set.normals[v_data.nidx].x as f32;
        normals[3*idx + 1] = obj_set.normals[v_data.nidx].y as f32;
        normals[3*idx + 2] = obj_set.normals[v_data.nidx].z as f32;
        idx += 1;
    }

    loader.load_animated_model_to_vao(&positions, &texture_coords, &indices, &normals, &joint_weights, &joint_indices)
}

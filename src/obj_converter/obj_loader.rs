use crate::math::{
    Matrix2f,
    Vector2f,
    Vector3f,
    Vector4f,
    utils::gram_schmidt_orthogonalize,
};
use std::io::{
    prelude::*,
    BufReader,
};
use std::fs::File;
use std::collections::HashMap;

type TanAndBitan = (Vector3f, Vector3f, usize);

const ERROR_MSG: &'static str = "Invalid .obj file. Verify that it contains faces";

pub fn load_simple_obj_model(file_name: &str) -> std::io::Result<ModelData> {
    load_obj_model(file_name, false)
}

pub fn load_obj_model(file_name: &str, compute_tangent: bool) -> std::io::Result<ModelData> {
    let obj_file = File::open(file_name)?;
    let buf_reader = BufReader::new(obj_file);
    
    let mut vertices = Vec::new();
    let mut textures = Vec::new();
    let mut normals = Vec::new();
    let mut tangents = Vec::new();
            
    let mut textures_sorted: Option<Vec<Vector2f>> = None;
    let mut normals_sorted: Option<Vec<Vector3f>> = None;
    
    let mut indices = Vec::new();

    // some of the faces we find may point at the same vertex index but we different normal/texture combination
    // this means that at that vertex location there are multiple vertices.
    // we need to be able to add these to our arrays of vertices/texture/normals/indices
    let mut vertex_dupes: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut extra_vertex_id_generator = 0;
    let mut furthest_distance = 0.0;

    for line in buf_reader.lines() {
        match line {
            Ok(content) => {
                let tokens: Vec<_> = content.split(" ").collect();
                if tokens.len() == 0 {
                    continue;
                }
                if tokens[0] == "v" {
                    let x = tokens[1].parse().unwrap();
                    let y = tokens[2].parse().unwrap();
                    let z = tokens[3].parse().unwrap();
                    vertices.push(Vector3f::new(x, y, z));
                    let dist = vertices[vertices.len()-1].length();
                    if furthest_distance < dist {
                        furthest_distance = dist;
                    }
                    tangents.push(new_tan_bitan());                    
                } else if tokens[0] == "vt" {
                    let u = tokens[1].parse::<f32>().unwrap();
                    let v = 1.0 - tokens[2].parse::<f32>().unwrap();
                    textures.push(Vector2f::new(u, v));
                } else if tokens[0] == "vn" {
                    let x = tokens[1].parse().unwrap();
                    let y = tokens[2].parse().unwrap();
                    let z = tokens[3].parse().unwrap();
                    normals.push(Vector3f::new(x, y, z));
                } else if tokens[0] == "f" {
                    // normals and texture coords are not in the order of vertices but we need them to be
                    // we use _sorted vectors to put them in the vertex order
                    let textures_mut_ref = textures_sorted.get_or_insert_with(|| vec![Vector2f::default(); vertices.len()]);
                    let normals_mut_ref = normals_sorted.get_or_insert_with(|| vec![Vector3f::default(); vertices.len()]);
                    
                    if extra_vertex_id_generator < vertices.len() {
                        extra_vertex_id_generator = vertices.len();
                    }

                    let mut face_vertices = [0usize; 3];                
                    for i in 1..4 {
                        face_vertices[i - 1] = process_face_token(tokens[i], textures_mut_ref, 
                            normals_mut_ref, &mut indices, &textures, &normals, 
                            &mut vertex_dupes, &mut extra_vertex_id_generator, &mut vertices, 
                            &mut tangents, file_name);
                    }
                    if compute_tangent {
                        calculate_tangents(face_vertices, &mut tangents, &vertices, textures_mut_ref);
                    }                    
                }
            },
            Err(e) => return Err(e),
        }
    };
    
    let flat_vertices = vertices.into_iter()
                                .flat_map(|v| v.into_iter())
                                .collect::<Vec<f32>>();
    let flat_textures = textures_sorted.expect(ERROR_MSG).into_iter()
                                       .flat_map(|v| v.into_iter())
                                       .collect::<Vec<f32>>();
    let flat_tangents = if compute_tangent {    
        tangents.into_iter()
                .enumerate()
                .map(|(idx, bitan)| update_tangent_with_handedness_and_average(bitan, &normals_sorted.as_ref().expect(ERROR_MSG)[idx])) 
                .flat_map(|v| v.into_iter())
                .collect::<Vec<f32>>()
    } else {
        Vec::new()
    };
    let flat_normals = normals_sorted.expect(ERROR_MSG).into_iter()
                                     .flat_map(|v| v.into_iter())
                                     .collect::<Vec<f32>>();
    
    
    Ok(ModelData{
        vertices: flat_vertices, 
        texture_coords: flat_textures, 
        normals: flat_normals, 
        indices,
        tangents: flat_tangents, 
        furthest_point: furthest_distance,
    })
}

fn update_tangent_with_handedness_and_average(tangent: TanAndBitan, normal: &Vector3f) -> Vector4f {    
    // gram-schmidt orthogonalize 
    // needed since tan and bitangent are not necessarily orthogonal from our calculation
    // however we want them to be orthogonal so that we can perform inversion of tangent space matrix
    // simply by transposing the matrix
    let (mut tan, bitan) = gram_schmidt_orthogonalize(&normal, tangent.0, tangent.1);    
    tan.normalize();

    let handedness = tan.cross_prod(normal).dot_product(&bitan);
    let handedness = if handedness < 0.0 {
        -1.0
    } else {
        1.0
    };

    Vector4f::new(tan.x, tan.y, tan.z, handedness)    
}

fn process_face_token(token: &str, textures_sorted: &mut Vec<Vector2f>, normals_sorted: &mut Vec<Vector3f>, indices: &mut Vec<u32>, 
                textures: &Vec<Vector2f>, normals: &Vec<Vector3f>, vertex_dupes: &mut HashMap<usize, Vec<(usize, usize, usize)>>,
                extra_vertex_id_gen: &mut usize, vertices: &mut Vec<Vector3f>, tangents: &mut Vec<TanAndBitan>, _file_name: &str) -> usize {
    let idx: Vec<_> = token.split("/").collect();
    let mut vertex_index = idx[0].parse::<usize>().expect(".obj didn't contain vertices") - 1;
    let texture_index = idx[1].parse::<usize>().expect(".obj didn't contain vt texture coords") - 1;
    let normal_index = idx[2].parse::<usize>().expect(".obj didn't contain normals") - 1;

    let tex_norm_tups: &mut Vec<(usize, usize, usize)> = vertex_dupes.entry(vertex_index).or_insert(Vec::new());    
    if let Some(&(texture, normal, vert_idx)) = tex_norm_tups.iter().find(|&&(t, n, _)| t == texture_index && n == normal_index) {
        // the vertex/texture/normal combination for this token already exists so use it
        vertex_index = vert_idx;        
        textures_sorted[vertex_index] = textures[texture].clone();
        normals_sorted[vertex_index] = normals[normal].clone();
    }
    else if tex_norm_tups.is_empty() {
        tex_norm_tups.push((texture_index, normal_index, vertex_index));
        textures_sorted[vertex_index] = textures[texture_index].clone();
        normals_sorted[vertex_index] = normals[normal_index].clone();
    }
    else {
        // we have found a duplicate -> generate new vertex id and new entries in vertices/normals/textures
        vertices.push(vertices[vertex_index].clone());
        tangents.push(new_tan_bitan());
        vertex_index = *extra_vertex_id_gen;
        tex_norm_tups.push((texture_index, normal_index, vertex_index));
        textures_sorted.push(textures[texture_index].clone());
        normals_sorted.push(normals[normal_index].clone());
        *extra_vertex_id_gen += 1;
    }
    indices.push(vertex_index as u32);
    vertex_index
}

fn new_tan_bitan() -> TanAndBitan {
    (Vector3f::zero(), Vector3f::zero(), 0)
}

fn calculate_tangents(triangle: [usize; 3], tangents: &mut Vec<TanAndBitan>, vertices: &Vec<Vector3f>, textures: &Vec<Vector2f>) {
    let mut st = Matrix2f::new();
    st[0][0] = textures[triangle[1]].x - textures[triangle[0]].x;
    st[0][1] = textures[triangle[1]].y - textures[triangle[0]].y;
    st[1][0] = textures[triangle[2]].x - textures[triangle[0]].x;
    st[1][1] = textures[triangle[2]].y - textures[triangle[0]].y;

    let stinv = st.inverse();
    
    let q1 = Vector3f::new(vertices[triangle[1]].x - vertices[triangle[0]].x, vertices[triangle[1]].y - vertices[triangle[0]].y, vertices[triangle[1]].z - vertices[triangle[0]].z);
    let q2 = Vector3f::new(vertices[triangle[2]].x - vertices[triangle[0]].x, vertices[triangle[2]].y - vertices[triangle[0]].y, vertices[triangle[2]].z - vertices[triangle[0]].z);
    
    let mut tangent1 = Vector3f::zero();
    let mut tangent2 = Vector3f::zero();    
    tangent1.x = stinv[0][0] * q1.x + stinv[0][1] * q2.x;
    tangent1.y = stinv[0][0] * q1.y + stinv[0][1] * q2.y;
    tangent1.z = stinv[0][0] * q1.z + stinv[0][1] * q2.z;

    tangent2.x = stinv[1][0] * q1.x + stinv[1][1] * q2.x;
    tangent2.y = stinv[1][0] * q1.y + stinv[1][1] * q2.y;
    tangent2.z = stinv[1][0] * q1.z + stinv[1][1] * q2.z;

    for id in 0..triangle.len() {
        tangents[triangle[id]].0 += &tangent1;
        tangents[triangle[id]].1 += &tangent2;
        tangents[triangle[id]].2 += 1;
    }
}

pub struct ModelData {
    pub vertices: Vec<f32>,
    pub texture_coords: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub tangents: Vec<f32>,
    pub furthest_point: f32,
}

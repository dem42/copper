use crate::math::{
    Vector2f,
    Vector3f,
};
use std::io::{
    prelude::*,
    BufReader,
};
use std::fs::File;
use std::collections::HashMap;

const ERROR_MSG: &'static str = "Invalid .obj file. Verify that it contains faces";

pub fn load_obj_model(file_name: &str) -> std::io::Result<ModelData> {
    let obj_file = File::open(file_name)?;
    let buf_reader = BufReader::new(obj_file);
    
    let mut vertices = Vec::new();
    let mut textures = Vec::new();
    let mut normals = Vec::new();
    
    let mut textures_sorted: Option<Vec<Vector2f>> = None;
    let mut normals_sorted: Option<Vec<Vector3f>> = None;
    let mut indices = Vec::new();
    let mut extra_vertex_id_gen = 0;
    let mut vertex_dupes: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();
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
                    let textures_mut_ref = textures_sorted.get_or_insert_with(|| vec![Vector2f::default(); vertices.len()]);
                    let normals_mut_ref = normals_sorted.get_or_insert_with(|| vec![Vector3f::default(); vertices.len()]);
                    if extra_vertex_id_gen < vertices.len() {
                        extra_vertex_id_gen = vertices.len();
                    }
                                        
                    for i in 1..4 {
                        process_token(tokens[i], textures_mut_ref, normals_mut_ref, &mut indices, &textures, &normals, 
                                    &mut vertex_dupes, &mut extra_vertex_id_gen, &mut vertices, file_name);
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
    let flat_normals = normals_sorted.expect(ERROR_MSG).into_iter()
                                     .flat_map(|v| v.into_iter())
                                     .collect::<Vec<f32>>();
    
    Ok(ModelData{
        vertices: flat_vertices, 
        texture_coords: flat_textures, 
        normals: flat_normals, 
        indices, 
        furthest_point: furthest_distance,
    })    
}

fn process_token(token: &str, textures_to_sort: &mut Vec<Vector2f>, normals_to_sort: &mut Vec<Vector3f>, indices: &mut Vec<u32>, 
                textures: &Vec<Vector2f>, normals: &Vec<Vector3f>, vertex_dupes: &mut HashMap<usize, Vec<(usize, usize, usize)>>,
                extra_vertex_id_gen: &mut usize, vertices: &mut Vec<Vector3f>, _file_name: &str) {
    let idx: Vec<_> = token.split("/").collect();
    let mut vertex_index = idx[0].parse::<usize>().expect(".obj didn't contain vertices") - 1;
    let texture_index = idx[1].parse::<usize>().expect(".obj didn't contain vt texture coords") - 1;
    let normal_index = idx[2].parse::<usize>().expect(".obj didn't contain normals") - 1;

    let tex_norm_tups: &mut Vec<(usize, usize, usize)> = vertex_dupes.entry(vertex_index).or_insert(Vec::new());    
    if let Some(&(texture, normal, vert_idx)) = tex_norm_tups.iter().find(|&&(t, n, _)| t == texture_index && n == normal_index) {        
        vertex_index = vert_idx;        
        textures_to_sort[vertex_index] = textures[texture].clone();
        normals_to_sort[vertex_index] = normals[normal].clone();
    }
    else if tex_norm_tups.is_empty() {
        tex_norm_tups.push((texture_index, normal_index, vertex_index));
        textures_to_sort[vertex_index] = textures[texture_index].clone();
        normals_to_sort[vertex_index] = normals[normal_index].clone();
    }
    else {        
        vertices.push(vertices[vertex_index].clone());
        vertex_index = *extra_vertex_id_gen;
        tex_norm_tups.push((texture_index, normal_index, vertex_index));
        textures_to_sort.push(textures[texture_index].clone());
        normals_to_sort.push(normals[normal_index].clone());
        *extra_vertex_id_gen += 1;
    }
    indices.push(vertex_index as u32);
}

pub struct ModelData {
    pub vertices: Vec<f32>,
    pub texture_coords: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub furthest_point: f32,
}

use super::loader::{
    ModelLoader,
    RawModel,
};
use super::math::{
    Vector2f,
    Vector3f,
};
use std::io::{
    prelude::*,
    BufReader,
};
use std::fs::File;

const ERROR_MSG: &'static str = "Invalid .obj file. Verify that it contains faces";

pub fn load_obj_model(file_name: &str, loader: &mut ModelLoader) -> std::io::Result<RawModel> {
    let obj_file = File::open(file_name)?;
    let buf_reader = BufReader::new(obj_file);
    
    let mut vertices = Vec::new();
    let mut textures = Vec::new();
    let mut normals = Vec::new();
    
    let mut textures_sorted: Option<Vec<Vector2f>> = None;
    let mut normals_sorted: Option<Vec<Vector3f>> = None;
    let mut indices = Vec::new();

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
                    for i in 1..4 {
                        process_token(tokens[i], textures_mut_ref, normals_mut_ref, &mut indices, &textures, &normals);
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
    let _flat_normals = normals_sorted.expect(ERROR_MSG).into_iter()
                                     .flat_map(|v| v.into_iter())
                                     .collect::<Vec<f32>>();
    
    Ok(loader.load_to_vao(&flat_vertices, &flat_textures, &indices))
}

fn process_token(token: &str, textures_to_sort: &mut Vec<Vector2f>, normals_to_sort: &mut Vec<Vector3f>, indices: &mut Vec<u32>, textures: &Vec<Vector2f>, normals: &Vec<Vector3f>) {
    let idx: Vec<_> = token.split("/").collect();
    let vertex_index = idx[0].parse::<usize>().unwrap() - 1;
    let texture_index = idx[1].parse::<usize>().unwrap() - 1;
    let normal_index = idx[2].parse::<usize>().unwrap() - 1;
    textures_to_sort[vertex_index] = textures[texture_index].clone();
    normals_to_sort[vertex_index] = normals[normal_index].clone();
    indices.push(vertex_index as u32);
}
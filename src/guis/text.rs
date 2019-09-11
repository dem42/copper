use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::rc::Rc;

use crate::math::{
    Vector2f,
    Vector3f,
};
use crate::models::{
    RawModel,
    TextureId,
};

#[derive(Debug)]
struct MetaFileCharDesc {
    character: char,
    pos: (i32, i32),
    size: (i32, i32),
    offset: (i32, i32),
    xadvance: i32,
}

struct MetaFile {
    atlas_size: (u32, u32),
    line_height: i32,
    char_map: HashMap<char, MetaFileCharDesc>,
} 

impl MetaFile {
    const DEFAULT_FONT_SIZE: f32 = 10.0;

    /**
     * parses a .fnt file created with Hiero
     */
    pub fn load_from_file(filename: &str) -> std::io::Result<MetaFile> {
        
        let fnt_file = File::open(filename)?;
        let buf_reader = BufReader::new(fnt_file);

        let mut char_map = HashMap::new();
        let line_iter = buf_reader.lines();
        let mut line_iter = line_iter.skip(1);
        let line_info = line_iter.next().expect(".fnt file must have line info on line 2").expect("unable to read second line from reader");
        let line_info_tokens = line_info.split_whitespace().collect::<Vec<_>>();
        
        let line_height = MetaFile::get_num_from_tkn(line_info_tokens[1]);
        let atlas_size_w = MetaFile::get_num_from_tkn(line_info_tokens[3]);
        let atlas_size_h = MetaFile::get_num_from_tkn(line_info_tokens[4]);        

        let premable_skipped = line_iter.skip(2);
        
        for char_line in premable_skipped {
            match char_line {
                Ok(content) => {
                    let tokens: Vec<_> = content.split_whitespace().collect();
                    if tokens.len() < 9 {
                        // char section is parsed
                        break;
                    }
                    let char_id: u8 = MetaFile::get_num_from_tkn(tokens[1]);
                    let char_id = char_id as char;
                    let x = MetaFile::get_num_from_tkn(tokens[2]);
                    let y = MetaFile::get_num_from_tkn(tokens[3]);
                    let width = MetaFile::get_num_from_tkn(tokens[4]);
                    let height = MetaFile::get_num_from_tkn(tokens[5]);
                    let xoffset = MetaFile::get_num_from_tkn(tokens[6]);
                    let yoffset = MetaFile::get_num_from_tkn(tokens[7]);
                    let xadvance = MetaFile::get_num_from_tkn(tokens[8]);

                    char_map.insert(char_id, MetaFileCharDesc{
                        character: char_id,
                        pos: (x, y),
                        size: (width, height),
                        offset: (xoffset, yoffset),
                        xadvance,
                    });
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }   

        Ok(MetaFile {
            char_map,
            line_height,
            atlas_size: (atlas_size_w, atlas_size_h),
        })
    }

    fn get_num_from_tkn<T>(str_token: &str) -> T 
        where T: std::str::FromStr,
              <T as std::str::FromStr>::Err: std::fmt::Debug
    {        
        let name_value: Vec<_> = str_token.split("=").collect();
        let val = name_value[1].parse::<T>().expect("Metafile must contain name=value pairs");
        val
    }

    fn scale_horiz(&self, horiz_val: i32) -> f32 {
        horiz_val as f32 / self.atlas_size.0 as f32
    }

    fn scale_vert(&self, vert_val: i32) -> f32 {
        vert_val as f32 / self.atlas_size.1 as f32
    }

    fn scale(&self, mut v: Vector2f, font_size: usize) -> Vector2f {
        let font_scale = font_size as f32 / MetaFile::DEFAULT_FONT_SIZE;
        v.x = (v.x * font_scale) / self.atlas_size.0 as f32;
        v.y = (v.y * font_scale) / self.atlas_size.1 as f32;
        v
    }
}

#[derive(Clone)]
pub struct FontType {
    meta_file: Rc<MetaFile>,
    pub texture_atlas: TextureId,
}

impl PartialEq for FontType {
    fn eq(&self, other: &FontType) -> bool {
        self.texture_atlas == other.texture_atlas
    }
}

impl Eq for FontType {}

impl Hash for FontType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.texture_atlas.hash(state);
    }
}

impl FontType {
    pub fn new(fnt_file_name: &str, texture_atlas_id: TextureId) -> FontType {
        let meta_file = MetaFile::load_from_file(fnt_file_name).expect(&format!("Unable to load fnt file: {}", fnt_file_name));
        FontType {
            meta_file: Rc::new(meta_file),
            texture_atlas: texture_atlas_id,
        }
    }
}

pub struct GuiText {
    pub font_type: FontType,
    pub text_model: RawModel,
    pub position: Vector2f,
    pub material: TextMaterial,
}

impl GuiText {
    pub fn new(font_type: FontType, text_model: RawModel, position: Vector2f, material: TextMaterial) -> GuiText {
        GuiText {
            font_type,
            text_model,
            position,
            material,        
        }
    }
}

pub struct TextMaterial {
    pub color: Vector3f,
    pub width: f32,
    pub edge: f32,
    pub outline_width: f32,
    pub outline_edge: f32,
    pub outline_color: Vector3f,
    pub offset: Vector2f,
}

impl Default for TextMaterial {
    fn default() -> Self {
        TextMaterial {
            color: Vector3f::new(1.0, 1.0, 1.0),
            width: 0.5,
            edge: 0.1,
            outline_width: 0.5,
            outline_edge: 0.4,
            outline_color: Vector3f::new(0.0, 1.0, 0.0),
            offset: Vector2f::new(0.0, 0.0),
        }
    }
}

pub mod text_mesh_creator {
    use super::*;
    
    #[derive(Debug)]
    pub struct TextMesh {
        pub positions: Vec<f32>,
        pub tex_coords: Vec<f32>,
        pub char_count: usize,
    }

    pub fn create_mesh(text: &str, font_type: &FontType, font_size: usize) -> TextMesh {

        let mut positions: Vec<Vector2f> = Vec::new();
        let mut tex_coords: Vec<Vector2f> = Vec::new();
        let mut line_pos_x = 0;
        let mut line_pos_y = font_type.meta_file.line_height;
        let mut char_cnt = 0;
        let mf = &font_type.meta_file;
        
        for line in text.lines() {            
            for c in line.chars() {
                let meta_data = mf.char_map.get(&c);
                if let Some(meta_data_val) = meta_data {
                    char_cnt += 1;
                    
                    // odd bug seen here where it either didnt re-compile or computed the signs incorrectly
                    // if you ever see misaligned graphemes check here
                    let top = (line_pos_y - meta_data_val.offset.1) as f32;
                    let left = (line_pos_x + meta_data_val.offset.0) as f32;
                    let right = left + meta_data_val.size.0 as f32;
                    let bottom = top - meta_data_val.size.1 as f32;

                    let l_tex = mf.scale_horiz(meta_data_val.pos.0);
                    let b_tex = mf.scale_vert(meta_data_val.pos.1);
                    let r_tex = mf.scale_horiz(meta_data_val.pos.0 + meta_data_val.size.0);
                    let t_tex = mf.scale_vert(meta_data_val.pos.1 + meta_data_val.size.1);
                    
                    let left_upper = mf.scale(Vector2f::new(left, top), font_size);
                    let left_lower = mf.scale(Vector2f::new(left, bottom), font_size);
                    let right_upper = mf.scale(Vector2f::new(right, top), font_size);
                    let right_lower = mf.scale(Vector2f::new(right, bottom), font_size);

                    let tex_lu = Vector2f::new(l_tex, b_tex);
                    let tex_ll = Vector2f::new(l_tex, t_tex);
                    let tex_ru = Vector2f::new(r_tex, b_tex);
                    let tex_rl = Vector2f::new(r_tex, t_tex);

                    // the order of the vertices is important since we have backface culling turned on
                    // backface culling means that triangles which are assumed to face away from camera are not to be rendered
                    // since we may be inside them
                    // here we go with counter-clockwise order                
                    // build position quad
                    positions.push(left_lower.clone());
                    positions.push(right_upper.clone());
                    positions.push(left_upper.clone());
                    
                    positions.push(right_upper.clone());
                    positions.push(left_lower.clone());
                    positions.push(right_lower.clone());

                    // build tex coord quad
                    tex_coords.push(tex_ll.clone());
                    tex_coords.push(tex_ru.clone());
                    tex_coords.push(tex_lu.clone());
                    
                    tex_coords.push(tex_ru.clone());
                    tex_coords.push(tex_ll.clone());
                    tex_coords.push(tex_rl.clone());

                    line_pos_x += meta_data_val.xadvance;
                }
            }
            line_pos_y -= font_type.meta_file.line_height;   
            line_pos_x = 0;
        }

        let flat_pos = positions.into_iter().flat_map(|v| v.into_iter()).collect::<Vec<f32>>();
        let flat_tex = tex_coords.into_iter().flat_map(|v| v.into_iter()).collect::<Vec<f32>>();

        TextMesh {
            positions: flat_pos,
            tex_coords: flat_tex,
            char_count: char_cnt,
        }
    }
}
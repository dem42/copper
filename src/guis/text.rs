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
};
use crate::models::{
    RawModel,
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
    line_height: u32,
    base: u32,
    char_map: HashMap<char, MetaFileCharDesc>,
} 

impl MetaFile {
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
        let base = MetaFile::get_num_from_tkn(line_info_tokens[2]);
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
            base,
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
}

#[derive(Clone)]
pub struct FontType {
    meta_file: Rc<MetaFile>,
    pub texture_atlas: u32,
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
    pub fn new(fnt_file_name: &str, texture_atlas_id: u32) -> FontType {
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
}

impl GuiText {
    pub fn new(font_type: FontType, text_model: RawModel) -> GuiText {
        GuiText {
            font_type,
            text_model,
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

    pub fn create_mesh(text: &str, font_type: &FontType) -> TextMesh {

        let mut positions: Vec<Vector2f> = Vec::new();
        let mut tex_coords: Vec<Vector2f> = Vec::new();
        let mut line_pos_x = 0;
        let line_pos_y = font_type.meta_file.base as i32;
        let mut char_cnt = 0;

        for c in text.chars() {
            let meta_data = font_type.meta_file.char_map.get(&c);
            if let Some(meta_data_val) = meta_data {
                char_cnt += 1;
                
                let bottom = (line_pos_y) as f32;
                let left = (line_pos_x + meta_data_val.offset.0) as f32;
                let right = left + meta_data_val.size.0 as f32;
                let top = bottom + meta_data_val.size.1 as f32;

                let l_tex = meta_data_val.pos.0 as f32 / font_type.meta_file.atlas_size.0 as f32;
                let b_tex = meta_data_val.pos.1 as f32 / font_type.meta_file.atlas_size.1 as f32;
                let r_tex = (meta_data_val.pos.0 + meta_data_val.size.0) as f32 / font_type.meta_file.atlas_size.0 as f32;
                let t_tex = (meta_data_val.pos.1 + meta_data_val.size.1) as f32 / font_type.meta_file.atlas_size.1 as f32;
                
                let left_upper = Vector2f::new(left / font_type.meta_file.atlas_size.0 as f32, top / font_type.meta_file.atlas_size.1 as f32);
                let left_lower = Vector2f::new(left / font_type.meta_file.atlas_size.0 as f32, bottom / font_type.meta_file.atlas_size.1 as f32);
                let right_upper = Vector2f::new(right / font_type.meta_file.atlas_size.0 as f32, top / font_type.meta_file.atlas_size.1 as f32);
                let right_lower = Vector2f::new(right / font_type.meta_file.atlas_size.0 as f32, bottom / font_type.meta_file.atlas_size.1 as f32);

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

        let flat_pos = positions.into_iter().flat_map(|v| v.into_iter()).collect::<Vec<f32>>();
        let flat_tex = tex_coords.into_iter().flat_map(|v| v.into_iter()).collect::<Vec<f32>>();

        TextMesh {
            positions: flat_pos,
            tex_coords: flat_tex,
            char_count: char_cnt,
        }
    }
}
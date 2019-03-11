use std::collections::HashMap;
use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::rc::Rc;

struct MetaFileCharDesc {
    character: char,
    pos: (i32, i32),
    size: (i32, i32),
    offset: (i32, i32),
    xadvance: i32,
}

struct MetaFile {
    pub char_map: HashMap<char, MetaFileCharDesc>,
} 

impl MetaFile {
    /**
     * parses a .fnt file created with Hiero
     */
    pub fn load_from_file(filename: &str) -> std::io::Result<MetaFile> {
        
        let fnt_file = File::open(filename)?;
        let buf_reader = BufReader::new(fnt_file);

        let mut char_map = HashMap::new();

        let premable_skipped = buf_reader.lines().skip(4);
        
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
    texture_atlas: u32,
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
    font_type: FontType,
    text_mesh_vao_id: u32,
    text_char_count: usize,
}


pub mod text_mesh_creator {
    use super::*;
    
    pub struct TextMesh {

    }

    pub fn create_mesh(text: &str, font_type: &FontType) -> TextMesh {

        for c in text.chars() {

        }

        unimplemented!()
    }
}
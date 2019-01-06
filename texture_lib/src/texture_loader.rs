use std::fmt;
use std::fs::File;
use std::io::Error;

pub struct Texture<PixelType> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<PixelType>,
}

impl<PixelType> Texture<PixelType> {
    pub fn get_color(&self, i: usize, j: usize) -> &PixelType {
        &self.data[i*self.width + j]
    }
}

impl<PixelType> fmt::Display for Texture<PixelType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(width:{}, height: {}, bytes: {})", self.width, self.height, self.data.len())
    }
}

pub struct RGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

pub type Texture2DRGBA = Texture<RGBA<u8>>;

fn create_color_type(buf: &[u8], color_type: &png::ColorType, i: usize) -> RGBA<u8> {
    if *color_type == png::ColorType::RGBA {
        RGBA {
            r: buf[4*i],
            g: buf[4*i+1], 
            b: buf[4*i+2], 
            a: buf[4*i+3], 
        }   
    } else {
        RGBA {
            r: buf[3*i],
            g: buf[3*i+1], 
            b: buf[3*i+2], 
            a: 255, 
        }   
    }    
}

pub fn load_rgba_2d_texture(file_name: &str, reverse: bool) -> Result<Texture2DRGBA, Error> {
    let decoder = png::Decoder::new(File::open(file_name)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    let mut result = Vec::new();
    let bytes_per_color = if info.color_type == png::ColorType::RGBA { 4 } else { 3 };
    let rbga_count = info.buffer_size() / bytes_per_color;

    println!("filename: {}. buffer size: {}. width: {}. height: {}. colorType: {:?}. bit_depth: {:?}. line_size: {}", 
        file_name, info.buffer_size(), info.width, info.height,
        info.color_type, info.bit_depth, info.line_size);
    println!("bytes_per_color: {}. rgba_count: {}", bytes_per_color, rbga_count);

    for i in 0..rbga_count {
        result.push(create_color_type(&buf, &info.color_type, i));
    }
    
    if reverse {
        // png loads from top to bottom
        // based on how we setup the coordinates for our model we may need to reverse
        result.reverse();
    }
    Ok(Texture{
        width: info.width as usize,
        height: info.height as usize,
        data: result,
    })
}
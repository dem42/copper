extern crate lodepng;

use std::fmt;
use lodepng::ffi::Error;

pub struct Texture<PixelType> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<PixelType>,
}

impl<PixelType> fmt::Display for Texture<PixelType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(width:{}, height: {}, bytes: {})", self.width, self.height, self.data.len())
    }
}

pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type Texture2DRGB = Texture<RGB<u8>>;

pub fn load_rgb_2d_texture(file_name: &str) -> Result<Texture2DRGB, Error> {
    let bitmap = lodepng::decode24_file(file_name)?;
    let mut result: Vec<_> = bitmap.buffer.iter().map(|&rgb| RGB {r: rgb.r, g: rgb.g, b: rgb.b}).collect();
    // lodepng loads from top to bottom. opengl expects bottom to top
    result.reverse();
    Ok(Texture{
        width: bitmap.width,
        height: bitmap.height,
        data: result,
    })
}
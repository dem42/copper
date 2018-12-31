use crate::math::{
    Vector3f,
};
use crate::models::{
    TexturedModel,
};

pub trait Transformable {
    fn position(&mut self) -> &mut Vector3f;
    fn rotation(&mut self) -> &mut Vector3f;
    fn scale(&mut self) -> &mut f32;
}

pub trait Renderable {
    fn model(&self) -> &TexturedModel;
}

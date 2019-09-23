use crate::math::{
    Matrix4f,
    Vector4f
};

pub enum CorrectionTransform {
    None,
    CoordinateSystemCorrection(Matrix4f),
}

impl CorrectionTransform {
    pub fn apply(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        match self {
            CorrectionTransform::None => (x, y, z),
            CorrectionTransform::CoordinateSystemCorrection(trans_mat) => {
                let mut temp_vec = Vector4f::new(x, y, z, 0.0);
                temp_vec = trans_mat.transform(&temp_vec);
                (temp_vec.x, temp_vec.y, temp_vec.z)
            },
        }
    } 
}
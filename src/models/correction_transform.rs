use crate::math::{
    Matrix4f,
    Vector4f
};

#[derive(Clone)]
pub enum CorrectionTransform {
    None,
    CoordinateSystemCorrection(Matrix4f, Matrix4f),
}

impl CorrectionTransform {
    pub fn create_coord_correction(transform: Matrix4f) -> CorrectionTransform {
        let itrans = transform.inverse();
        CorrectionTransform::CoordinateSystemCorrection(transform, itrans)
    }

    pub fn apply(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        match self {
            CorrectionTransform::None => (x, y, z),
            CorrectionTransform::CoordinateSystemCorrection(trans_mat, _) => {
                let mut temp_vec = Vector4f::new(x, y, z, 0.0);
                temp_vec = trans_mat.transform(&temp_vec);
                (temp_vec.x, temp_vec.y, temp_vec.z)
            },
        }
    }

    pub fn apply_to_bind_transform(&self, transform_mat: &mut Matrix4f) {
        match self {
            CorrectionTransform::None => {},
            CorrectionTransform::CoordinateSystemCorrection(trans_mat, _) => {                
                transform_mat.pre_multiply_in_place(trans_mat);
            },
        }
    }

    pub fn apply_to_inverse_bind_transform(&self, transform_mat: &mut Matrix4f) {
        match self {
            CorrectionTransform::None => {},
            CorrectionTransform::CoordinateSystemCorrection(_, itrans_mat) => {                
                transform_mat.post_multiply_in_place(&itrans_mat);
            },
        }
    } 
}
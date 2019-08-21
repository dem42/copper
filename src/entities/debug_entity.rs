use crate::models::{    
    DynamicVertexIndexedModel,
};
use crate::math::{
    Vector3f,
};

pub struct DebugEntity {
    pub model: DynamicVertexIndexedModel,
    pub position: Vector3f,
    pub rotation: Vector3f,
    pub scale: Vector3f,
}

impl DebugEntity {
    pub fn new(model: DynamicVertexIndexedModel) -> Self {
        Self {
            model,
            position: Vector3f::zero(),
            rotation: Vector3f::zero(),
            scale: Vector3f { x: 1.0, y: 1.0, z: 1.0 },
        }
    }
}
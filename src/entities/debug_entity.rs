use crate::models::{    
    DynamicVertexIndexedModel,
};

pub struct DebugEntity {
    pub model: DynamicVertexIndexedModel,
}

impl DebugEntity {
    pub fn new(model: DynamicVertexIndexedModel) -> Self {
        Self {
            model,
        }
    }
}
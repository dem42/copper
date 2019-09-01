use crate::entities::{
    Entity,
    Camera,
    Light,
    Player,
    Ground,
    Skybox,
    WaterTile,
    DebugEntity,
};
use crate::math::Vector3f;
use crate::models::QuadModel;
use crate::guis::{
    GuiPanel,
    GuiText,
};
use crate::particles::AdvancedParticleSystem;

pub struct Scene {
    pub entities: Vec<Entity>, 
    pub normal_mapped_entities: Vec<Entity>, 
    pub ground: Ground, 
    pub player: Player, 
    // a simple quad model used for guis and post processing
    pub quad_model: QuadModel, 
    pub water: Vec<WaterTile>,
    pub debug_entity: DebugEntity,
    pub camera: Camera,
    pub skybox: Skybox,
    pub texts: Vec<GuiText>,
    pub guis: Vec<GuiPanel>,
    pub lights: Vec<Light>,
    pub particle_systems: Vec<(AdvancedParticleSystem, Vector3f)>,
}

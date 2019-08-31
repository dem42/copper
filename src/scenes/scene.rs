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
use crate::models::GuiModel;
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
    pub gui_model: GuiModel, 
    pub water: Vec<WaterTile>,
    pub debug_entity: DebugEntity,
    pub camera: Camera,
    pub skybox: Skybox,
    pub texts: Vec<GuiText>,
    pub guis: Vec<GuiPanel>,
    pub lights: Vec<Light>,
    pub particle_systems: Vec<(AdvancedParticleSystem, Vector3f)>,
}

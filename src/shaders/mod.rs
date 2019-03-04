pub mod shader_program;
pub mod static_shader;
pub mod terrain_shader;
pub mod gui_shader;
pub mod skybox_shader;
pub mod water_shader;
pub mod normal_map_static_shader;

pub use self::static_shader::StaticShader;
pub use self::normal_map_static_shader::NormalMapStaticShader;
pub use self::terrain_shader::TerrainShader;
pub use self::gui_shader::GuiShader;
pub use self::skybox_shader::SkyboxShader;
pub use self::water_shader::WaterShader;
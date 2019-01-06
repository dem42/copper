pub mod shader_program;
pub mod static_shader;
pub mod terrain_shader;
pub mod gui_shader;
pub mod skybox_shader;

pub use self::static_shader::StaticShader;
pub use self::terrain_shader::TerrainShader;
pub use self::gui_shader::GuiShader;
pub use self::skybox_shader::SkyboxShader;
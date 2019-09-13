use crate::models::{
    ResourceManager,
    QuadModel,
};
use crate::math::{
    Vector2f,
    Vector3f,
};
use crate::guis::{
    GuiPanel,
    GuiText,
    TextMaterial,
};

pub struct LoadScreen {
    pub guis: Vec<GuiPanel>,
    pub texts: Vec<GuiText>,
    pub gui_model: QuadModel,
}

pub fn init_resourced_for_load_screen(resource_manager: &mut ResourceManager) {    
    resource_manager.init_quad_model();
    resource_manager.init_gui_textures();

    resource_manager.init_fonts();
}

pub fn create_load_screen(resource_manager: &mut ResourceManager) -> LoadScreen {
    let gui_background = resource_manager.get_gui_texture(ResourceManager::WHITE_TEXTURE);
    //let shadow_map = framebuffers.shadowmap_fbo.depth_texture;
    let guis = vec!{
        GuiPanel::new(gui_background, Vector2f::new(0.0, 0.0), Vector2f::new(1.0, 1.0)),
    };

    let texts = vec![
        resource_manager.create_gui_text("Loading the world...", 
            ResourceManager::COPPER_SDF_FONT_TYPE, 4, Vector2f::new(-0.25, 0.0), 
            TextMaterial {
                color: Vector3f::new(1.0, 0.0, 0.0), 
                width: 0.5, edge: 0.3,
                outline_width: 0.5, outline_edge: 0.4,
                ..TextMaterial::default()
            }
        ),
    ];

    LoadScreen {
        guis,
        texts,
        gui_model: resource_manager.quad_model(),
    }
}
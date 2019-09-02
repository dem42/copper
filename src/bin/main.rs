extern crate copper;

use copper::display::{
    Display,
    framebuffers::FboMap,
    framebuffers::FramebufferObject,
    framebuffers::FboFlags,
};
use copper::renderers::{
    master_renderer::MasterRenderer,
    gui_renderer::GuiRenderer,
};
use copper::models::{
    ResourceManager,
};
use copper::particles::{
    ParticleMaster,
};
use copper::post_processing::post_processing::PostProcessing;
use copper::mouse_picker::MousePicker;
use copper::scenes::{
    scene::Scene,
    all_scene::create_scene
};

fn main() {
    let mut display = Display::create();
    let mut framebuffers = FboMap::new(&display);
    let mut resource_manager = ResourceManager::default();

    let mut scene = create_scene(&mut resource_manager, &framebuffers);
    
    let mut master_renderer = MasterRenderer::new(&display.projection_matrix, display.get_aspect_ratio());
    let mut gui_renderer = GuiRenderer::new();
    
    let mut mouse_picker = MousePicker::new();

    // particle effects master
    let mut particle_master = ParticleMaster::new(&display.projection_matrix);
    let mut post_processing = PostProcessing::new(scene.quad_model.clone(), &display);

    const POSTPROCESSING: bool = true;
        
    while !display.is_close_requested() {

        scene.camera.move_camera(&display, &scene.player);
        
        update_mouse_picker_and_move_lamp(&mut mouse_picker, &display, &mut scene);

        spin_around_normal_mapped_entities(&mut scene, &display);
        
        particle_master.emit_particles(&scene.particle_systems, &display);
        
        particle_master.update(&display, &scene.camera);

        scene.player.move_player(&display, &scene.ground);

        scene.skybox.increase_rotation(&display);

        master_renderer.render(&scene.lights, &mut scene.camera, &scene.entities, &scene.normal_mapped_entities, &scene.ground.terrains, 
            &scene.player, &scene.water, &scene.skybox, &display, &mut framebuffers, &mut particle_master, &mut scene.debug_entity);

        if POSTPROCESSING {
            do_anti_aliasing_for_fbo(&mut framebuffers, &display);

            post_processing.do_post_processing(&mut framebuffers, &display);
        } else {
            do_anti_aliasing_to_screen(&mut framebuffers, &display);
        }

        gui_renderer.render(&scene.guis, &scene.quad_model.raw_model, &scene.texts);

        display.update_display();
    }
}

fn do_anti_aliasing_for_fbo(framebuffers: &mut FboMap, display: &Display) {
    let display_size = display.get_size();
    let camera_multisampled_fbo = framebuffers.fbos.get_mut(FboMap::CAMERA_TEXTURE_FBO_MULTI).expect("A multisampled fbo must be present MSAA processing of camera output");
    // create the target fbo that will later be read from in post processing shaders
    let mut camera_texture_fbo = FramebufferObject::new(display_size.0 as usize, display_size.1 as usize, FboFlags::COLOR_TEX | FboFlags::DEPTH_TEX);

    camera_multisampled_fbo.resolve_to_fbo(&mut camera_texture_fbo, display);

    framebuffers.insert(FboMap::CAMERA_TEXTURE_FBO, camera_texture_fbo);
}

fn do_anti_aliasing_to_screen(framebuffers: &mut FboMap, display: &Display) {
    let camera_multisampled_fbo = framebuffers.fbos.get_mut(FboMap::CAMERA_TEXTURE_FBO_MULTI).expect("A multisampled fbo must be present MSAA processing of camera output");
    camera_multisampled_fbo.resolve_to_screen(&display);
}

fn update_mouse_picker_and_move_lamp(mouse_picker: &mut MousePicker, display: &Display, scene: &mut Scene) {
    if let Some(selected_pos) = mouse_picker.update(&display, &display.projection_matrix, &scene.camera, &scene.ground) {            
        let last_pos = scene.entities.len()-1;
        scene.entities[last_pos].set_position(&selected_pos);
        scene.lights[3].position = selected_pos;
        scene.lights[3].position.y += 14.0; 
    }
}

fn spin_around_normal_mapped_entities(scene: &mut Scene, display: &Display) {
    const SPEED: f32 = 20.0;
    for idx in 0..scene.normal_mapped_entities.len() {
        scene.normal_mapped_entities[idx].increase_rotation(0.0, 0.0, SPEED * display.frame_time_sec);
    }
}
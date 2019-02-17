use std::collections::HashMap;
use crate::display::{
    Display,
    Framebuffers,
    FramebufferObject,
    WallClock,
};
use crate::gl;
use crate::entities::{
    Entity,
    Camera,
    Light,
    Terrain,
    Player,
    Skybox,
    WaterTile,
};
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,
};
use crate::models::{
    TexturedModel,
};
use super::entity_renderer::EntityRenderer;
use super::terrain_renderer::TerrainRenderer;
use super::skybox_renderer::SkyboxRenderer;
use super::water_renderer::WaterRenderer;

pub struct BatchRenderer {    
    entity_renderer: EntityRenderer,
    terrain_renderer: TerrainRenderer,
    skybox_renderer: SkyboxRenderer,
    water_renderer: WaterRenderer,
    pub projection_matrix: Matrix4f,
}

impl BatchRenderer {

    const FOV_HORIZONTAL: f32 = 70.0;
    // here using actual world coords which are RHS coord sys with z axis going into screen (so more negative means further)
    const NEAR: f32 = -0.1;
    const FAR: f32 = -1000.0;
    const SKY_COLOR: Vector3f = Vector3f{ x: 0.5444, y: 0.62, z: 0.69 };

    pub fn new(display: &Display) -> BatchRenderer {
        let projection_matrix = Matrix4f::create_projection_matrix(BatchRenderer::NEAR, BatchRenderer::FAR, BatchRenderer::FOV_HORIZONTAL, display.get_aspect_ration());
        let entity_renderer = EntityRenderer::new(&projection_matrix);
        let terrain_renderer = TerrainRenderer::new(&projection_matrix);
        let skybox_renderer = SkyboxRenderer::new(&projection_matrix);
        let water_renderer = WaterRenderer::new(&projection_matrix);
        
        BatchRenderer {
            entity_renderer,
            terrain_renderer,
            skybox_renderer,
            water_renderer,
            projection_matrix,
        }
    }
    
    pub fn render(&mut self, lights: &Vec<Light>, camera: &mut Camera, entities: &Vec<Entity>, terrains: &Vec<Terrain>, player: &Player, water_tiles: &Vec<WaterTile>, skybox: &Skybox, display: &Display, framebuffers: &Framebuffers) {
        // enable clip plane                    
        gl::enable(gl::CLIP_DISTANCE0); 

        let water_height = WaterTile::get_water_height(water_tiles);
        let above_water_clip_plane = Vector4f::new(0.0, -1.0, 0.0, water_height);
        let below_water_clip_plane = Vector4f::new(0.0, 1.0, 0.0, -water_height);
        let above_infinity_plane = Vector4f::new(0.0, -1.0, 0.0, 10_000.0);
        
        camera.set_to_reflected_ray_camera_origin(water_height);
        framebuffers.reflection_fbo.bind();
        self.render_pass(lights, camera, entities, terrains, player, skybox, &display.wall_clock, &below_water_clip_plane);
        camera.set_to_reflected_ray_camera_origin(water_height);

        // we should also move camera before refraction to account for refracted angle?
        framebuffers.refraction_fbo.bind();
        self.render_pass(lights, camera, entities, terrains, player, skybox, &display.wall_clock, &above_water_clip_plane);

        gl::disable(gl::CLIP_DISTANCE0); // apparently this doesnt work on all drivers?
        display.restore_default_framebuffer();
        self.render_pass(lights, camera, entities, terrains, player, skybox, &display.wall_clock, &above_infinity_plane);
        // render water
        self.water_renderer.render(water_tiles, camera);
    }

    fn render_pass(&mut self, lights: &Vec<Light>, camera: &Camera, entities: &Vec<Entity>, terrains: &Vec<Terrain>, player: &Player, skybox: &Skybox, wall_clock: &WallClock, clip_plane: &Vector4f) {
        self.prepare();

        // render entites
        self.entity_renderer.start_render(lights, camera, &BatchRenderer::SKY_COLOR);
        let groups_by_tex = BatchRenderer::group_entities_by_tex(entities);
        for (textured_model, entity_vec) in groups_by_tex.iter() {
            self.entity_renderer.prepare_textured_model(textured_model, clip_plane);
            for entity in entity_vec {
                // load transform matrix into shader
                self.entity_renderer.render(entity);
            }
            self.entity_renderer.unprepare_textured_model(textured_model);
        }        
        // render player
        self.entity_renderer.prepare_textured_model(&player.entity.model, clip_plane); 
        self.entity_renderer.render(&player.entity);
        self.entity_renderer.unprepare_textured_model(&player.entity.model);

        self.entity_renderer.stop_render();

        // render terrain
        self.terrain_renderer.start_render(lights, camera, &BatchRenderer::SKY_COLOR);
        for terrain in terrains.iter() {
            self.terrain_renderer.prepare_terrain(terrain, clip_plane);
            self.terrain_renderer.render(terrain);
            self.terrain_renderer.unprepare_terrain();
        }
        self.terrain_renderer.stop_render();

        self.skybox_renderer.render(camera, skybox, &BatchRenderer::SKY_COLOR, wall_clock, clip_plane);
    }
    
    fn prepare(&self) {
        gl::helper::enable_backface_culling();
        gl::enable(gl::DEPTH_TEST);
        let (Vector3f{x : r, y : g, z : b}, a) = (BatchRenderer::SKY_COLOR, 1.0);
        gl::clear_color(r, g, b, a);
        gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    fn group_entities_by_tex<'b>(entities: &'b Vec<Entity>) -> HashMap<&'b TexturedModel, Vec<&'b Entity>> {
        let mut groups_by_tex = HashMap::new();

        for entity in entities.iter() {
            let group = groups_by_tex.entry(&entity.model).or_insert(Vec::new());
            group.push(entity);
        }

        groups_by_tex
    }
}
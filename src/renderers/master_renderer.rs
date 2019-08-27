use std::collections::HashMap;
use crate::display::{
    Display,
    Framebuffers,
    FramebufferObject,
    WallClock,
};
use crate::gl;
use crate::entities::*;
use crate::math::{
    Matrix4f,
    Vector3f,
    Vector4f,
};
use crate::models::{
    TexturedModel,
};
use crate::shadows::shadowmap_renderer::ShadowMapRenderer;
use super::entity_renderer::EntityRenderer;
use super::normal_map_entity_renderer::NormalMapEntityRenderer;
use super::terrain_renderer::TerrainRenderer;
use super::skybox_renderer::SkyboxRenderer;
use super::water_renderer::WaterRenderer;
use super::debug_renderer::DebugRenderer;

pub struct RenderGroup {
    pub id: u32,
    pub name: &'static str,
}

impl RenderGroup {    
    pub const SHADOW_MAP_PASS: RenderGroup = RenderGroup {id: 0, name: "ShadowMapPass"};
    pub const REFLECT_REFRACT_PASS: RenderGroup = RenderGroup {id: 1, name: "ReflectRefractPass"};
    pub const DRAW_ENTITIES: RenderGroup = RenderGroup {id: 2, name: "EntityDrawPass"};
    pub const DRAW_NORMAL_MAP_ENTITIES: RenderGroup = RenderGroup {id: 3, name: "NormalMapEntityDrawPass"};
    pub const DRAW_TERRAIN: RenderGroup = RenderGroup {id: 4, name: "TerrainDraw"};
    pub const DRAW_SKYBOX: RenderGroup = RenderGroup {id: 5, name: "Skybox"};
    pub const DRAW_WATER: RenderGroup = RenderGroup {id: 6, name: "WaterSurfaceDraw"};
    pub const PARTICLE_EFFECTS_PASS: RenderGroup = RenderGroup {id: 7, name: "ParticleEffects"};
    pub const DRAW_GUI: RenderGroup = RenderGroup {id: 8, name: "GuiOverlayDraw"};
}

pub struct MasterRenderer {    
    entity_renderer: EntityRenderer,
    normal_map_entity_renderer: NormalMapEntityRenderer,
    terrain_renderer: TerrainRenderer,
    skybox_renderer: SkyboxRenderer,
    water_renderer: WaterRenderer,
    shadowmap_renderer: ShadowMapRenderer,
    debug_renderer: DebugRenderer,
}

impl MasterRenderer {

    const SKY_COLOR: Vector3f = Vector3f{ x: 0.5444, y: 0.62, z: 0.69 };

    pub fn new(projection_matrix: &Matrix4f, aspect_ratio: f32) -> MasterRenderer {
        let entity_renderer = EntityRenderer::new(projection_matrix);
        let normal_map_entity_renderer = NormalMapEntityRenderer::new(projection_matrix);
        let terrain_renderer = TerrainRenderer::new(projection_matrix);
        let skybox_renderer = SkyboxRenderer::new(projection_matrix);
        let water_renderer = WaterRenderer::new(projection_matrix, &MasterRenderer::SKY_COLOR);
        let shadowmap_renderer = ShadowMapRenderer::new(aspect_ratio);
        let debug_renderer = DebugRenderer::new(projection_matrix);

        MasterRenderer {
            entity_renderer,
            normal_map_entity_renderer,
            terrain_renderer,
            skybox_renderer,
            water_renderer,
            shadowmap_renderer,
            debug_renderer,
        }
    }
    
    pub fn render(&mut self, lights: &Vec<Light>, camera: &mut Camera, entities: &Vec<Entity>, normal_mapped_entities: &Vec<Entity>, terrains: &Vec<Terrain>, 
                player: &Player, water_tiles: &Vec<WaterTile>, skybox: &Skybox, display: &Display, framebuffers: &mut Framebuffers, debug_entity: &mut DebugEntity) {

        self.do_shadowmap_render_passes(camera, framebuffers, entities, normal_mapped_entities, player, lights, terrains);

        self.do_water_render_passes(water_tiles, camera, framebuffers, entities, normal_mapped_entities, terrains, player, lights, skybox, display);
        display.restore_default_framebuffer();

        let above_infinity_plane = Vector4f::new(0.0, -1.0, 0.0, 10_000.0);
        self.render_pass(lights, camera, entities, normal_mapped_entities, terrains, player, skybox, &display.wall_clock, &above_infinity_plane, framebuffers.shadowmap_fbo.depth_texture);
        // render water
        self.water_renderer.render(water_tiles, framebuffers, camera, display, lights);

        //let obb_ref = &self.shadowmap_renderer.shadow_box.frustum_corners;
        //self.debug_renderer.render(debug_entity, camera, obb_ref); 
        debug_entity.position = self.shadowmap_renderer.shadow_box.world_space_center.clone();
        debug_entity.scale = Vector3f::new(100.0, 100.0, 100.0);
        //debug_entity.scale = 0.80 * Vector3f::new(self.shadowmap_renderer.shadow_box.width, self.shadowmap_renderer.shadow_box.height, self.shadowmap_renderer.shadow_box.length);
        //self.debug_renderer.render_cube(debug_entity, camera);
    }

    fn do_shadowmap_render_passes(&mut self, camera: &mut Camera, framebuffers: &mut Framebuffers, entities: &Vec<Entity>, 
                normal_mapped_entities: &Vec<Entity>, player: &Player, lights: &Vec<Light>, _terrains: &Vec<Terrain>) {
        
        gl::helper::push_debug_group(RenderGroup::SHADOW_MAP_PASS.id, RenderGroup::SHADOW_MAP_PASS.name);

        framebuffers.shadowmap_fbo.bind();
        self.shadowmap_renderer.start_render(camera, &lights[0]);

        // render into the shadowmap depth buffer all the entities that we want to cast shadows
        let entity_by_tex = MasterRenderer::group_entities_by_tex(entities);
        for (tex_model, entity_group) in entity_by_tex {
            self.shadowmap_renderer.prepare_textured_model(tex_model);
            self.shadowmap_renderer.render(&entity_group);
            self.shadowmap_renderer.cleanup_textured_model();
        }

        let norm_entity_by_tex = MasterRenderer::group_entities_by_tex(normal_mapped_entities);
        for (tex_model, entity_group) in norm_entity_by_tex {
            self.shadowmap_renderer.prepare_textured_model(tex_model);
            self.shadowmap_renderer.render(&entity_group);
            self.shadowmap_renderer.cleanup_textured_model();
        }

        self.shadowmap_renderer.prepare_textured_model(&player.entity.model);
        self.shadowmap_renderer.render_entity(&player.entity);
        self.shadowmap_renderer.cleanup_textured_model();

        self.shadowmap_renderer.stop_render();

        gl::helper::pop_debug_group();
    }

    fn do_water_render_passes(&mut self, water_tiles: &Vec<WaterTile>, camera: &mut Camera, framebuffers: &mut Framebuffers,
                entities: &Vec<Entity>, normal_mapped_entities: &Vec<Entity>, terrains: &Vec<Terrain>, player: &Player, lights: &Vec<Light>,
                skybox: &Skybox, display: &Display) {

        if water_tiles.is_empty() {
            return;
        }

        gl::helper::push_debug_group(RenderGroup::REFLECT_REFRACT_PASS.id, RenderGroup::REFLECT_REFRACT_PASS.name);
        // enable clip plane                    
        gl::enable(gl::CLIP_DISTANCE0); 

        let water_height = WaterTile::get_water_height(water_tiles);
        let tiny_overlap = 0.07; // to prevent glitches near the edge of the water
        let above_water_clip_plane = Vector4f::new(0.0, -1.0, 0.0, water_height + tiny_overlap);
        let below_water_clip_plane = Vector4f::new(0.0, 1.0, 0.0, -water_height + tiny_overlap);        
        
        camera.set_to_reflected_ray_camera_origin(water_height);
        framebuffers.reflection_fbo.bind();
        self.render_pass(lights, camera, entities, normal_mapped_entities, terrains, player, skybox, &display.wall_clock, &below_water_clip_plane, framebuffers.shadowmap_fbo.depth_texture);
        camera.set_to_reflected_ray_camera_origin(water_height);

        // we should also move camera before refraction to account for refracted angle?
        framebuffers.refraction_fbo.bind();
        self.render_pass(lights, camera, entities, normal_mapped_entities, terrains, player, skybox, &display.wall_clock, &above_water_clip_plane, framebuffers.shadowmap_fbo.depth_texture);

        gl::disable(gl::CLIP_DISTANCE0); // apparently this doesnt work on all drivers?   

        gl::helper::pop_debug_group();     
    }

    fn render_pass(&mut self, lights: &Vec<Light>, camera: &Camera, entities: &Vec<Entity>, normal_mapped_entities: &Vec<Entity>, terrains: &Vec<Terrain>, 
                player: &Player, skybox: &Skybox, wall_clock: &WallClock, clip_plane: &Vector4f, shadow_map_texture: u32) {

        gl::helper::push_debug_group(RenderGroup::DRAW_ENTITIES.id, RenderGroup::DRAW_ENTITIES.name);
        self.prepare();

        // render entites
        self.entity_renderer.start_render(lights, camera, &MasterRenderer::SKY_COLOR);
        let groups_by_tex = MasterRenderer::group_entities_by_tex(entities);
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
        gl::helper::pop_debug_group();     

        gl::helper::push_debug_group(RenderGroup::DRAW_NORMAL_MAP_ENTITIES.id, RenderGroup::DRAW_NORMAL_MAP_ENTITIES.name);
        // render normal mapped entites
        self.normal_map_entity_renderer.start_render(lights, camera, &MasterRenderer::SKY_COLOR);
        let groups_by_tex = MasterRenderer::group_entities_by_tex(normal_mapped_entities);
        for (textured_model, entity_vec) in groups_by_tex.iter() {
            self.normal_map_entity_renderer.prepare_textured_model(textured_model, clip_plane);
            for entity in entity_vec {
                // load transform matrix into shader
                self.normal_map_entity_renderer.render(entity);
            }
            self.normal_map_entity_renderer.unprepare_textured_model(textured_model);
        }
        self.normal_map_entity_renderer.stop_render(); 
        gl::helper::pop_debug_group();

        // render terrain
        gl::helper::push_debug_group(RenderGroup::DRAW_TERRAIN.id, RenderGroup::DRAW_TERRAIN.name);
        self.terrain_renderer.start_render(lights, camera, &MasterRenderer::SKY_COLOR, self.shadowmap_renderer.get_to_shadow(), shadow_map_texture);
        for terrain in terrains.iter() {
            self.terrain_renderer.prepare_terrain(terrain, clip_plane);
            self.terrain_renderer.render(terrain);
            self.terrain_renderer.unprepare_terrain();
        }
        self.terrain_renderer.stop_render();
        gl::helper::pop_debug_group();

        gl::helper::push_debug_group(RenderGroup::DRAW_SKYBOX.id, RenderGroup::DRAW_SKYBOX.name);
        self.skybox_renderer.render(camera, skybox, &MasterRenderer::SKY_COLOR, wall_clock, clip_plane);
        gl::helper::pop_debug_group();
    }
    
    fn prepare(&self) {
        gl::helper::enable_backface_culling();
        gl::enable(gl::DEPTH_TEST);
        let (Vector3f{x : r, y : g, z : b}, a) = (MasterRenderer::SKY_COLOR, 1.0);
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
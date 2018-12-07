extern crate copper;
use copper::display::Display;
use copper::renderer::Renderer;
use copper::loader::{
    ModelLoader,
    TexturedModel,    
};
use copper::shaders::shader_program::{
    StaticShader,
};
use copper::entities::Entity;
use copper::math::Vector3f;
use copper::camera::Camera;

fn test_engine() {
    let mut display = Display::create();    
    let mut loader = ModelLoader::new();
    
    let vertices = vec!{			
            -0.5,0.5,-0.5,	
            -0.5,-0.5,-0.5,	
            0.5,-0.5,-0.5,	
            0.5,0.5,-0.5,		
            
            -0.5,0.5,0.5,	
            -0.5,-0.5,0.5,	
            0.5,-0.5,0.5,	
            0.5,0.5,0.5,
            
            0.5,0.5,-0.5,	
            0.5,-0.5,-0.5,	
            0.5,-0.5,0.5,	
            0.5,0.5,0.5,
            
            -0.5,0.5,-0.5,	
            -0.5,-0.5,-0.5,	
            -0.5,-0.5,0.5,	
            -0.5,0.5,0.5,
            
            -0.5,0.5,0.5,
            -0.5,0.5,-0.5,
            0.5,0.5,-0.5,
            0.5,0.5,0.5,
            
            -0.5,-0.5,0.5,
            -0.5,-0.5,-0.5,
            0.5,-0.5,-0.5,
            0.5,-0.5,0.5				
    };
		
    let tex_coords = vec!{				
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0,			
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0,			
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0,
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0,
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0,
            0.0,0.0,
            0.0,1.0,
            1.0,1.0,
            1.0,0.0				
    };
    
    let indices = vec!{
            0,1,3,	
            3,1,2,	
            4,5,7,
            7,5,6,
            8,9,11,
            11,9,10,
            12,13,15,
            15,13,14,	
            16,17,19,
            19,17,18,
            20,21,23,
            23,21,22
    };

    let raw_model = loader.load_to_vao(&vertices, &tex_coords, &indices);
    let texture = loader.load_texture("res/textures/test.png");
    let textured_model = TexturedModel { raw_model, texture };
    let mut shader = StaticShader::new(&textured_model.raw_model);

    let renderer = Renderer::new(&display, &mut shader);

    let mut entity = Entity::new(textured_model, Vector3f::new(0.0,0.0,-5.0), Vector3f::new(0.0, 0.0, 0.0), 1.0);

    let mut camera = Camera::default();

    while !display.is_close_requested() {
        entity.increase_rotation(1.0, 1.0, 0.0);
        camera.move_camera(&display);

        renderer.prepare();
        shader.start();
        shader.load_view_matrix(&camera);
        renderer.render(&entity, &mut shader);
        shader.stop();
        display.update_display();
    }
}

fn main() {
    test_engine();    
}

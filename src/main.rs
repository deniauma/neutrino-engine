pub mod graphics;
pub mod procedural;
pub mod server;
use std::time::{Instant};
use crate::graphics::mesh::*;
use crate::graphics::shader::*;
use crate::graphics::transform::Transform;
use crate::graphics::states::*;
use crate::graphics::entity::*;
use crate::graphics::inputs::*;
use crate::procedural::*;

struct GameEntity {
    freq: f64,
}

impl EntityState for GameEntity {
    fn on_create(&mut self, data: &mut graphics::ComponentStorageManager){

    }

    fn on_update(&mut self, data: GameData, delta: f32){
        let (id, storage, input) = data;
        /* let mesh = storage.get_mut_mesh(id).unwrap();
        self.freq += delta as f64;
        let map = heigth_map(100, 100, self.freq);
        *mesh = generate_mesh(&map); */
        let camera = storage.get_mut_camera();
        let speed = 0.1;
        if input.is_key_pressed(Key::Z) == ButtonState::PRESSED {
            camera.position.z -= speed;
        }
        if input.is_key_pressed(Key::S) == ButtonState::PRESSED {
            camera.position.z += speed;
        }
        if input.is_key_pressed(Key::Q) == ButtonState::PRESSED {
            camera.position.x -= speed;
        }
        if input.is_key_pressed(Key::D) == ButtonState::PRESSED {
            camera.position.x += speed;
        }
        
    }

    fn on_delete(&mut self, data: &mut graphics::ComponentStorageManager){

    }
}

fn example1() {
    let mut engine = graphics::Engine::new();
    engine.init();

    let mut mesh_builder = MeshBuilder::new();
    mesh_builder.add_vertex(0.5, 0.5, 0.0).add_vertex(0.5, -0.5, 0.0).add_vertex(-0.5, 0.5, 0.0);
    mesh_builder.add_vertex(0.5, -0.5, 0.0).add_vertex(-0.5, -0.5, 0.0).add_vertex(-0.5, 0.5, 0.0);
    mesh_builder.add_color(Color::new(1.0, 0.0, 0.0)).add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(1.0, 1.0, 0.0));
    mesh_builder.add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(0.0, 0.0, 1.0)).add_color(Color::new(1.0, 1.0, 0.0));
    mesh_builder.add_uv(UV::new(1.0, 1.0)).add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 1.0));
    mesh_builder.add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 1.0));
    //mesh_builder.auto_index();
    let rectangle = engine.create_scene_object();

    engine.add_mesh(
        rectangle.id,
        mesh_builder.commit(),
    );
    engine.add_material(
        rectangle.id,
        MaterialBuilder::simple_texture_material_2d("container.jpg"),
    );
    engine.add_transform(rectangle.id, Transform::new_default());

    struct GameEntity {
        id: graphics::Index
    }

    impl EntityState for GameEntity {
        fn on_create(&mut self, data: &mut graphics::ComponentStorageManager){

        }

        fn on_update(&mut self, data: GameData, delta: f32){
            let (id, storage, input) = data;
            let trans = storage.get_mut_transform(self.id).unwrap();
            //trans.rotation.set_x(45.0);
            trans.translation.x = 1.5;
            trans.scale.x = 1.0;
            trans.scale.y = 1.0;
            if input.is_key_pressed(Key::Space) == ButtonState::PRESSED {
                trans.rotation.y +=  delta*20.0;
                trans.rotation.x +=  delta*20.0;
            }
            let camera = storage.get_mut_camera();
            if input.is_key_pressed(Key::Z) == ButtonState::PRESSED {
                camera.position.z -= 1.0;
            }
            if input.is_key_pressed(Key::S) == ButtonState::PRESSED {
                camera.position.z += 1.0;
            }
            if input.is_key_pressed(Key::Q) == ButtonState::PRESSED {
                camera.position.x -= 1.0;
            }
            if input.is_key_pressed(Key::D) == ButtonState::PRESSED {
                camera.position.x += 1.0;
            }
            
        }

        fn on_delete(&mut self, data: &mut graphics::ComponentStorageManager){

        }
    }

    //using EntityBuilder
    let mut entity_builder = EntityBuilder::new();
    //entity_builder.with_quad_mesh(1.0);
    entity_builder.with_cube_mesh(1.0).with_texture("container.jpg");
    entity_builder.build(&mut engine);

    engine.add_states(2, GameEntity{id: 2});

    engine.start();
}

fn example2() {
    let map = heigth_map(100, 100, 5.0);
    // println!("Height map: {:?}", map);
    let mut engine = graphics::Engine::new();
    engine.init();
    let mut entity_builder = EntityBuilder::new();
    let start = Instant::now();
    let id = entity_builder.with_mesh(generate_mesh(&map)).build(&mut engine);
    let elapsed_time = start.elapsed();
    println!("Time to generate terrain: {} ms", elapsed_time.as_millis());
    engine.add_states(id, GameEntity{ freq: 1.0 });
    engine.start_debug_server();
    engine.start();
}

fn example3() {
    let mut engine = graphics::Engine::new();
    engine.init();
    let mut entity_builder = EntityBuilder::new();
    let id = entity_builder.with_cube_mesh(1.0).build(&mut engine);
    engine.start();
}

fn main() { 
    // example1();
    // example2();
    example3();
}

pub mod graphics;
use crate::graphics::mesh::*;
use crate::graphics::shader::*;
use crate::graphics::transform::Transform;
use crate::graphics::states::*;
use crate::graphics::entity::*;
use crate::graphics::inputs::*;


fn main() {
    let mut engine = graphics::Engine::new();
    engine.init();

    let mut mesh_builder = MeshBuilder::new();
    mesh_builder.add_vertex(Vertex::new(0.5, 0.5, 0.0)).add_vertex(Vertex::new(0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, 0.5, 0.0));
    mesh_builder.add_vertex(Vertex::new(0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, 0.5, 0.0));
    mesh_builder.add_color(Color::new(1.0, 0.0, 0.0)).add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(1.0, 1.0, 0.0));
    mesh_builder.add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(0.0, 0.0, 1.0)).add_color(Color::new(1.0, 1.0, 0.0));
    mesh_builder.add_uv(UV::new(1.0, 1.0)).add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 1.0));
    mesh_builder.add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 1.0));
    mesh_builder.auto_index();
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

        fn on_update(&mut self, data: &mut graphics::ComponentStorageManager, input: &InputSystem, delta: f32){
            let trans = data.get_mut_transform(self.id).unwrap();
            //trans.rotation.set_x(45.0);
            trans.translation.x = 1.5;
            trans.scale.x = 1.0;
            trans.scale.y = 1.0;
            if input.is_key_pressed(Key::Space) == ButtonState::PRESSED {
                trans.rotation.y +=  delta*20.0;
                trans.rotation.x +=  delta*20.0;
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

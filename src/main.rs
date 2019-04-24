pub mod graphics;
use crate::graphics::mesh::*;
use crate::graphics::shader::*;
use crate::graphics::transform::Transform;
use crate::graphics::states::*;
use crate::graphics::entity::*;


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

    /*let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];
    let positions = vec![
        Vertex::new(0.5, 0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(-0.5, 0.5, 0.0),
    ];
    let colors = vec![
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 1.0, 0.0),
        Color::new(0.0, 0.0, 1.0),
        Color::new(1.0, 1.0, 0.0),
    ];
    let text_coords = vec![
        UV::new(1.0, 1.0),
        UV::new(1.0, 0.0),
        UV::new(0.0, 0.0),
        UV::new(0.0, 1.0),
    ];*/
    engine.add_mesh(
        rectangle.id,
        //Mesh::new(positions, colors, text_coords, indices),
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

        fn on_update(&mut self, data: &mut graphics::ComponentStorageManager){
            let trans = data.get_mut_transform(self.id).unwrap();
            //trans.rotation.z = 45.0;
            //trans.rotation.set_x(45.0);
            trans.translation.x = 2.0;
            trans.scale.x = 1.0;
            trans.scale.y = 1.0;
        }

        fn on_delete(&mut self, data: &mut graphics::ComponentStorageManager){

        }
    }

    engine.add_states(rectangle.id, GameEntity{id: rectangle.id});

    //using EntityBuilder
    let mut entity_builder = EntityBuilder::new();
    entity_builder.with_quad_mesh(1.0);
    entity_builder.build(&mut engine);

    engine.start();
}

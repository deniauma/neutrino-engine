pub mod graphics;
use crate::graphics::mesh::*;
use crate::graphics::shader::*;
extern crate math;
use math::*;

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

    engine.start();
}

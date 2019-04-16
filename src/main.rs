pub mod graphics;
use crate::graphics::mesh::*;
use crate::graphics::shader::*;
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

    struct Actor {
        id: graphics::Index,
        shader: Shader
    }

    impl Actor {
        pub fn new(id: u32, shader: Shader) -> Self {
            Self {
                id: id,
                shader: shader,
            }
        }
    }

    impl graphics::SceneUpdate for Actor {
        fn update(&self) {
            let entity = self.id;
            let mut trans = Mat4::new_identity();
            //trans = transforms::translate(trans, Vec3::new(1.0, 0.0, 0.0));
            //trans = transforms::scale(trans, Vec3::new(1.0, 2.0, 0.0));
            let angle: f32 = -45.0;
            trans = transforms::rotate(trans, Vec3::new(0.0, 0.0, 1.0), angle.to_radians());
            //println!("Trans: {:?}", trans);
            self.shader.set_mat4("transform", trans);
        }
    }

    let actor = Actor::new(rectangle.id, engine.get_material(rectangle.id).shader);
    engine.add_update(rectangle.id, Box::new(actor));

    engine.start();
}

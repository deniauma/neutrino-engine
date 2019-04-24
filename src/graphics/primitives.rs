use crate::graphics::{Mesh, Vertex, Color, UV, MeshBuilder};

#[derive(Debug)]
pub struct PrimitiveBuilder {

}

impl PrimitiveBuilder {
    pub fn quad() -> Mesh {
        let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];
        let positions = vec![
            Vertex::new(0.5, 0.5, 0.0),
            Vertex::new(0.5, -0.5, 0.0),
            Vertex::new(-0.5, -0.5, 0.0),
            Vertex::new(-0.5, 0.5, 0.0),
        ];
        let colors = vec![
        Color::new(1.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
        ];
        let uvs = vec![
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 0.0),
            UV::new(0.0, 1.0),
        ];

        Mesh::new(positions, colors, uvs, indices)
    }

    pub fn cube() -> Mesh {
        let positions = vec![
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),

            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(0.5, -0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),
            Vertex::new(-0.5, -0.5,  0.5),

            Vertex::new(-0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),

            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5, -0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),

            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5, -0.5,  0.5),
            Vertex::new(0.5, -0.5,  0.5),
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(-0.5, -0.5, -0.5),

            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5, -0.5),
        ];

        let uvs = vec![
            UV::new(0.0, 0.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0),

            UV::new(0.0, 0.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0),

            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0),
            UV::new(1.0, 0.0),

            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0),
            UV::new(1.0, 0.0),

            UV::new(0.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 0.0),
            UV::new(0.0, 1.0),

            UV::new(0.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 0.0),
            UV::new(0.0, 1.0)
        ];
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.add_vertex_from_vec(positions);
        mesh_builder.add_uv_from_vec(uvs);
        mesh_builder.commit()
    }
}
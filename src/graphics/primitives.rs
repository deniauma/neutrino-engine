use crate::graphics::{Mesh, Vertex, Color, UV, MeshBuilder, Normal};

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
        let normals = vec![
            Normal {x: 0.0, y: 1.0, z: 0.0},
            Normal {x: 0.0, y: 1.0, z: 0.0},
            Normal {x: 0.0, y: 1.0, z: 0.0},
            Normal {x: 0.0, y: 1.0, z: 0.0},
        ];

        Mesh::new(positions, colors, uvs, normals, indices)
    }

    pub fn cube(color: Color) -> Mesh {
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
        let norms = vec![
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:-1.0},

            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},
            cgmath::Vector3 {x:0.0, y:0.0, z:1.0},

            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:-1.0, y:0.0, z:0.0},

            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},
            cgmath::Vector3 {x:1.0, y:0.0, z:0.0},

            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:-1.0, z:0.0},

            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
            cgmath::Vector3 {x:0.0, y:1.0, z:0.0},
        ];
        let mut mesh_builder = MeshBuilder::new();
        for _ in 0..positions.len() {
            mesh_builder.add_color(color);
        }
        mesh_builder.add_vertex_from_vec(positions);
        mesh_builder.add_uv_from_vec(uvs);
        mesh_builder.add_normal_from_vec(norms);
        // mesh_builder.gen_vertex_normal();
        mesh_builder.commit()
    }
}
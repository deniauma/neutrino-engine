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

    pub fn plane(color: Color) -> Mesh {
        let positions = vec![
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new( 0.5,  0.5, -0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(-0.5,  0.5,  0.5),
        ];
        let uvs = vec![
            UV::new(0.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0)
        ];
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.add_vertex_from_vec(positions);
        mesh_builder.add_uv_from_vec(uvs);
        for _ in 0..6 {
            mesh_builder.add_color(color);
            mesh_builder.add_normal(Normal {x: 0.0, y: 1.0, z: 0.0});
        }
        mesh_builder.commit()
    }

    // https://learnopengl.com/code_viewer.php?code=advanced/faceculling_vertexdata
    pub fn cube(color: Color) -> Mesh {
        let positions = vec![
            // back face
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(0.5, 0.5, -0.5),
            Vertex::new(0.5,  -0.5, -0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, 0.5, -0.5),

            // front face
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(0.5, -0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),
            Vertex::new(-0.5, -0.5,  0.5),

            // left face
            Vertex::new(-0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),

            // right face
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5,  0.5, -0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(0.5,  0.5,  0.5),
            Vertex::new(0.5, -0.5,  0.5),

            // bottom face
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new( 0.5, -0.5, -0.5),
            Vertex::new( 0.5, -0.5,  0.5),
            Vertex::new( 0.5, -0.5,  0.5),
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new(-0.5, -0.5, -0.5),

            // top face
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new( 0.5,  0.5, -0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5, -0.5),
            Vertex::new(-0.5,  0.5,  0.5),
        ];

        let uvs = vec![
            UV::new(0.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 0.0),
            UV::new(0.0, 1.0),

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
            UV::new(0.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(0.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 0.0),

            UV::new(0.0, 1.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 0.0),
            UV::new(0.0, 1.0),

            UV::new(0.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(1.0, 1.0),
            UV::new(1.0, 0.0),
            UV::new(0.0, 1.0),
            UV::new(0.0, 0.0)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_normals() {
        let cube = PrimitiveBuilder::cube(Color::new(1.0, 0.5, 0.31));
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.add_vertex_from_vec(cube.positions);
        mesh_builder.add_color_from_vec(cube.colors);
        mesh_builder.add_uv_from_vec(cube.texture_coords);
        mesh_builder.gen_vertex_normal();
        // println!("{:?}\n{:?}", cube.normals, mesh_builder.commit().normals);
        assert_eq!(cube.normals, mesh_builder.commit().normals);
    }
}
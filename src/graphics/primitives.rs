use crate::graphics::{Mesh, Vertex, Color, UV};

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
}
#[derive(Debug,Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32,) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: 1.0
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct UV {
    u: f32,
    v: f32
}

impl UV {
    pub fn new(u: f32, v: f32) -> Self {
        Self {
            u: u,
            v: v
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshBuilder {
    mesh: Mesh,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            mesh: Mesh::new_empty(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> &mut Self {
        self.mesh.positions.push(vertex);
        self
    }

    pub fn add_color(&mut self, color: Color) -> &mut Self {
        self.mesh.colors.push(color);
        self
    }

    pub fn add_uv(&mut self, uv: UV) -> &mut Self {
        self.mesh.texture_coords.push(uv);
        self
    }

    pub fn add_index(&mut self, index: u32) -> &mut Self {
        self.mesh.indices.push(index);
        self
    }

    pub fn auto_index(&mut self) -> &mut Self {
        self.mesh.indices.clear();
        let positions = &self.mesh.positions;
        let mut index = 0;
        let mut cache: Vec<(Vertex, u32)> = Vec::new();
        let mut vertices: Vec<Vertex> = vec!();
        let mut indices: Vec<u32> = vec!();
        let mut colors: Vec<Color> = vec!();
        let mut texture_coords: Vec<UV> = vec!();
        for (i, pos) in positions.iter().enumerate() {
            //self.index(*pos);
            let mut id = 0;
            for (k, (vert, ind)) in cache.iter().enumerate() {
                if *pos == *vert {
                    id = *ind;
                    break;
                }
            }
            if id == 0 {
                id = index;
                index += 1;
                cache.push((*pos, id));
                vertices.push(*pos);
                colors.push(self.mesh.colors[i]);
                texture_coords.push(self.mesh.texture_coords[i]);
            }
            indices.push(id);
        }
        println!("New vertex data: {:?}", vertices);
        println!("New index data: {:?}", indices);
        let new_mesh = Mesh::new(vertices, colors, texture_coords, indices);
        self.mesh = new_mesh;
        self
    }

    pub fn commit(&self) -> Mesh {
        self.mesh.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub positions: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub texture_coords: Vec<UV>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new_empty() -> Mesh {
        Mesh {
            vertices: vec![],
            positions: vec![],
            colors: vec![],
            texture_coords: vec![],
            indices: vec![],
        }
    }

    pub fn new(vertex_coords: Vec<Vertex>, colors: Vec<Color>, texture_coords: Vec<UV>, vertex_indices: Vec<u32>) -> Mesh {
        Mesh {
            vertices: Mesh::build_vertices(&vertex_coords, &colors, &texture_coords),
            positions: vertex_coords,
            colors: colors,
            texture_coords: texture_coords,
            indices: vertex_indices,
        }
    }

    fn build_vertices(positions: &Vec<Vertex>, colors: &Vec<Color>, uvs: &Vec<UV>) -> Vec<f32> {
        let mut vertices: Vec<f32> = vec![];
        for (i, pos) in positions.iter().enumerate() {
            vertices.push(pos.x);
            vertices.push(pos.y);
            vertices.push(pos.z);
            vertices.push(colors[i].r);
            vertices.push(colors[i].g);
            vertices.push(colors[i].b);
            vertices.push(colors[i].a);
            vertices.push(uvs[i].u);
            vertices.push(uvs[i].v);
        }

        return vertices
    }

    pub fn get_vertices_data(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn size_of_vertex() -> usize {
        std::mem::size_of::<Vertex>()
    }

    pub fn size_of_vertices(&self) -> usize {
        self.vertices.len() * std::mem::size_of::<f32>()
    }

    pub fn size_of_indice() -> usize {
        std::mem::size_of::<u32>()
    }

    pub fn size_of_indices(&self) -> usize {
        self.indices.len() * Mesh::size_of_indice()
    }

    pub fn size_of_stride(&self) -> usize {
        let mut stride = Self::size_of_vertex(); // 3 float per vertex
        if self.colors.len() > 0 {
            stride += std::mem::size_of::<Color>(); // 4 float per color
        }
        if self.texture_coords.len() > 0 {
            stride += std::mem::size_of::<UV>(); // 2 float per UV
        }

        return stride
    }

    pub fn nb_elements_per_vertex() -> i32 {
        3
    }

    pub fn nb_elements_per_color() -> i32 {
        4
    }

    pub fn nb_elements_per_uv() -> i32 {
        2
    }

    pub fn vertex_offset(&self) -> usize {
        0
    }

    pub fn color_offset(&self) -> usize {
        self.vertex_offset() + Self::size_of_vertex()
    }

    pub fn uv_offset(&self) -> usize {
        self.color_offset() + std::mem::size_of::<Color>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_builder_auto_index() {
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.add_vertex(Vertex::new(0.5, 0.5, 0.0)).add_vertex(Vertex::new(0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, 0.5, 0.0));
        mesh_builder.add_vertex(Vertex::new(0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, -0.5, 0.0)).add_vertex(Vertex::new(-0.5, 0.5, 0.0));
        mesh_builder.add_color(Color::new(1.0, 0.0, 0.0)).add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(1.0, 1.0, 0.0));
        mesh_builder.add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(0.0, 0.0, 1.0)).add_color(Color::new(1.0, 1.0, 0.0));
        mesh_builder.add_uv(UV::new(1.0, 1.0)).add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 1.0));
        mesh_builder.add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 1.0));
        mesh_builder.auto_index();
        let mesh = mesh_builder.commit();
        let indices: Vec<u32> = vec![0, 1, 2, 1, 3, 2];
        assert_eq!(mesh.indices, indices);
    }
}

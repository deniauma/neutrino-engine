use cgmath::Vector3;
use cgmath::prelude::*;

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
    index: u32,
    cache: Vec<(Vertice, u32)>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            mesh: Mesh::new_empty(),
            index: 0,
            cache: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, x: f32, y:f32, z: f32) -> &mut Self {
        self.mesh.positions.push(Vertex::new(x, y, z));
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

    pub fn add_normal(&mut self, norm: Normal) -> &mut Self {
        self.mesh.normals.push(norm);
        self
    }

    pub fn add_index(&mut self, index: u32) -> &mut Self {
        self.mesh.indices.push(index);
        self
    }

    pub fn add_vertex_from_vec(&mut self, vertex: Vec<Vertex>) -> &mut Self {
        for v in vertex.iter() {
            self.mesh.positions.push(*v);
        }
        self
    }

    pub fn add_color_from_vec(&mut self, color: Vec<Color>) -> &mut Self {
        for c in color.iter() {
            self.mesh.colors.push(*c);
        }
        self
    }

    pub fn add_uv_from_vec(&mut self, uv: Vec<UV>) -> &mut Self {
        for u in uv.iter() {
            self.mesh.texture_coords.push(*u);
        }
        self
    }

    pub fn add_normal_from_vec(&mut self, norm: Vec<Normal>) -> &mut Self {
        for u in norm.iter() {
            self.mesh.normals.push(*u);
        }
        self
    }

    pub fn add_full_vertice_info(&mut self, pos: Vertex, color: Color, uv: UV, norm: Normal) -> &mut Self {
        let mut index = self.index;
        let mut new_vert: Vertice = [0.0;12];
        new_vert[0] = pos.x;
        new_vert[1] = pos.y;
        new_vert[2] = pos.z;
        new_vert[3] = color.r;
        new_vert[4] = color.g;
        new_vert[5] = color.b;
        new_vert[6] = color.a;
        new_vert[7] = uv.u;
        new_vert[8] = uv.v;
        new_vert[9] = norm.x;
        new_vert[10] = norm.y;
        new_vert[11] = norm.z;
        for (_, (vert, ind)) in self.cache.iter().enumerate() {
            if new_vert == *vert {
                index = *ind;
                break;
            }
        }
        if index == self.index {
            self.cache.push((new_vert, index));
            self.add_vertex(pos.x, pos.y, pos.z);
            self.add_color(color);
            self.add_uv(uv);
            self.index += 1;
        }
        self.add_index(index);
        
        self
    }

    pub fn gen_vertex_normal(&mut self) {
        for i in 0..self.mesh.positions.len()/3 {
            let n = Self::get_triangle_normal(self.mesh.positions[i*3], self.mesh.positions[i*3 + 1], self.mesh.positions[i*3 + 2]);
            for _ in 0..3 {
                self.add_normal(n);
            }
        }
    }

    pub fn get_triangle_normal(a: Vertex, b : Vertex, c: Vertex) -> Normal {
        let ab = Vector3 {x: b.x - a.x, y: b.y - a.y, z: b.z - a.z};
        let ac = Vector3 {x: c.x - a.x, y: c.y - a.y, z: c.z - a.z};
        ab.cross(ac).normalize()
    }

    /* pub fn auto_index(&mut self) -> &mut Self {
        self.mesh.indices.clear();
        let positions = &self.mesh.positions;
        let mut index = 0;
        let mut cache: Vec<(Vertice, u32)> = Vec::new();
        let mut vertices: Vec<Vertex> = vec!();
        let mut indices: Vec<u32> = vec!();
        let mut colors: Vec<Color> = vec!();
        let mut texture_coords: Vec<UV> = vec!();
        for (i, pos) in positions.iter().enumerate() {
            let mut id = index;
            let complete_vertex = self.mesh.build_vertex(i);
            for (_, (vert, ind)) in cache.iter().enumerate() {
                if complete_vertex == *vert {
                    id = *ind;
                    break;
                }
            }
            if id == index {
                index += 1;
                cache.push((complete_vertex, id));
                vertices.push(*pos);
                colors.push(self.mesh.colors[i]);
                texture_coords.push(self.mesh.texture_coords[i]);
            }
            indices.push(id);
        }
        println!("Before indexing: {}, after indexing: {}", positions.len(), vertices.len());
        let new_mesh = Mesh::new(vertices, colors, texture_coords, indices);
        self.mesh = new_mesh;
        self
    } */

    pub fn commit(&mut self) -> Mesh {
        if self.mesh.colors.is_empty() {
            let white = Color::new(1.0, 1.0, 1.0);
            for _ in 0..self.mesh.positions.len() {
                self.mesh.colors.push(white);
            }
        }
        if self.mesh.normals.is_empty() {
            let n = Normal {x: 0.0, y: 1.0, z: 0.0};
            for _ in 0..self.mesh.positions.len() {
                self.mesh.normals.push(n);
            }
        }
        /* if self.mesh.indices.is_empty() {
            let start = std::time::Instant::now();
            self.auto_index();
            println!("Time to auto index mesh w/o indices: {} ms", start.elapsed().as_millis());
        } */
        if self.mesh.vertices.is_empty() {
            self.mesh.vertices = Mesh::build_vertices(&self.mesh.positions, &self.mesh.colors, &self.mesh.texture_coords, &self.mesh.normals);
        }
        /* println!("New index data: {:?}", self.mesh.indices);
        println!("Nb of vertices: {:?}", self.mesh.vertices.len()); */
        self.mesh.clone()
    }
}

pub type Vertice = [f32;12];
pub type Normal = cgmath::Vector3<f32>;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub positions: Vec<Vertex>,
    pub colors: Vec<Color>,
    pub texture_coords: Vec<UV>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
    pub dirty: bool,
}

impl Mesh {
    pub fn new_empty() -> Mesh {
        Mesh {
            vertices: vec![],
            positions: vec![],
            colors: vec![],
            texture_coords: vec![],
            normals: vec![],
            indices: vec![],
            dirty: true,
        }
    }

    pub fn new(vertex_coords: Vec<Vertex>, colors: Vec<Color>, texture_coords: Vec<UV>, normals: Vec<Normal>, vertex_indices: Vec<u32>) -> Mesh {
        Mesh {
            vertices: Mesh::build_vertices(&vertex_coords, &colors, &texture_coords, &normals),
            positions: vertex_coords,
            colors: colors,
            texture_coords: texture_coords,
            normals: normals,
            indices: vertex_indices,
            dirty: true,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn build_vertex(&self, index: usize) -> Vertice {
        let mut vert: Vertice = [0.0;12];

        vert[0] = self.positions[index].x;
        vert[1] = self.positions[index].y;
        vert[2] = self.positions[index].z;
        vert[3] = self.colors[index].r;
        vert[4] = self.colors[index].g;
        vert[5] = self.colors[index].b;
        vert[6] = self.colors[index].a;
        vert[7] = self.texture_coords[index].u;
        vert[8] = self.texture_coords[index].v;
        vert[9] = self.normals[index].x;
        vert[10] = self.normals[index].y;
        vert[11] = self.normals[index].z;

        return vert
    }

    fn build_vertices(positions: &Vec<Vertex>, colors: &Vec<Color>, uvs: &Vec<UV>, normals: &Vec<Normal>) -> Vec<f32> {
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
            vertices.push(normals[i].x);
            vertices.push(normals[i].y);
            vertices.push(normals[i].z);
        }

        return vertices
    }

    pub fn get_vertices_data(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn print_vertices(&self) -> String {
        let mut s = String::from("");
        for i in 0..self.vertices.len()/12 {
            s.push_str(&format!("{}, {}, {}  {}, {}, {}, {}  {}, {}  {}, {}, {}\n", self.vertices[i*12], self.vertices[i*12+1], self.vertices[i*12+2], self.vertices[i*12+3], self.vertices[i*12+4], self.vertices[i*12+5], self.vertices[i*12+6], self.vertices[i*12+7], self.vertices[i*12+8], self.vertices[i*12+9], self.vertices[i*12+10], self.vertices[i*12+11]));
        }
        s
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
        if self.normals.len() > 0 {
            stride += std::mem::size_of::<Normal>(); // 3 float per normal
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

    pub fn nb_elements_per_normal() -> i32 {
        3
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

    pub fn normal_offset(&self) -> usize {
        self.uv_offset() + std::mem::size_of::<UV>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_builder_auto_index() {
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.add_vertex(0.5, 0.5, 0.0).add_vertex(0.5, -0.5, 0.0).add_vertex(-0.5, 0.5, 0.0);
        mesh_builder.add_vertex(0.5, -0.5, 0.0).add_vertex(-0.5, -0.5, 0.0).add_vertex(-0.5, 0.5, 0.0);
        mesh_builder.add_color(Color::new(1.0, 0.0, 0.0)).add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(1.0, 1.0, 0.0));
        mesh_builder.add_color(Color::new(0.0, 1.0, 0.0)).add_color(Color::new(0.0, 0.0, 1.0)).add_color(Color::new(1.0, 1.0, 0.0));
        mesh_builder.add_uv(UV::new(1.0, 1.0)).add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 1.0));
        mesh_builder.add_uv(UV::new(1.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 1.0));
        //mesh_builder.auto_index();
        let mesh = mesh_builder.commit();
        let indices: Vec<u32> = vec![0, 1, 2, 1, 3, 2];
        assert_eq!(mesh.indices, indices);
    }
}

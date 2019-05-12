extern crate noise;

use noise::{Perlin, NoiseFn};
use crate::graphics::mesh::{MeshBuilder, Vertex, Mesh, Color, UV};


pub fn heigth_map(width: usize, height: usize, freq: f64) -> Vec<Vec<f64>> {
    let mut map: Vec<Vec<f64>> = Vec::new();
    let perlin = Perlin::new();
    //let freq = 5.0;
    for x in 0 .. width {
        map.insert(x, Vec::new());
        for y in 0 .. height {
            let nx = (x as f64 / width as f64) - 0.5;
            let ny = (y as f64 / height as f64) - 0.5;
            map[x].insert(y, perlin.get([freq * nx, freq * ny]) / 2.0 + 0.5);
        }
    }
    map
}

pub fn generate_mesh(map: &Vec<Vec<f64>>) -> Mesh {
    let precision = 0.1;
    let mut mesh_builder = MeshBuilder::new();
    for x in 0..std::cmp::max(map.len()-1, 1) {
        for z in 0..std::cmp::max(map[x].len()-1, 1) {
            let vy = map[x][z] as f32;
            let vx = x as f32 * precision;
            let vz = 0.0 - z as f32 * precision;
            let vy2 = map[x + 1][z] as f32;
            let vy3 = map[x + 1][z + 1] as f32;
            let vy4 = map[x][z + 1] as f32;
            let vx2 = vx + precision;
            let vz2 =  vz - precision;
            mesh_builder.add_vertex(vx, vy, vz).add_vertex(vx2, vy2, vz).add_vertex(vx2, vy3, vz2);
            mesh_builder.add_vertex(vx2, vy3, vz2).add_vertex(vx, vy4, vz2).add_vertex(vx, vy, vz);
            mesh_builder.add_color(Color::new(vy, vy, vy)).add_color(Color::new(vy2, vy2, vy2)).add_color(Color::new(vy3, vy3, vy3));
            mesh_builder.add_color(Color::new(vy3, vy3, vy3)).add_color(Color::new(vy4, vy4, vy4)).add_color(Color::new(vy, vy, vy));
            mesh_builder.add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0));
            mesh_builder.add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0));
        }
    }
    // mesh_builder.auto_index();
    mesh_builder.commit()
}

pub fn generate_mesh_v2(map: &Vec<Vec<f64>>) -> Mesh {
    let precision = 0.1;
    let mut mesh_builder = MeshBuilder::new();
    for x in 0..std::cmp::max(map.len()-1, 1) {
        for z in 0..std::cmp::max(map[x].len()-1, 1) {
            let vy = map[x][z] as f32;
            let vx = x as f32 * precision;
            let vz = 0.0 - z as f32 * precision;
            let vy2 = map[x + 1][z] as f32;
            let vy3 = map[x + 1][z + 1] as f32;
            let vy4 = map[x][z + 1] as f32;
            let vx2 = vx + precision;
            let vz2 =  vz - precision;
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy, vz), Color::new(vy, vy, vy), UV::new(0.0, 0.0));
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy2, vz), Color::new(vy2, vy2, vy2), UV::new(0.0, 0.0));
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy3, vz2), Color::new(vy3, vy3, vy3), UV::new(0.0, 0.0));
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy3, vz2), Color::new(vy3, vy3, vy3), UV::new(0.0, 0.0));
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy4, vz2), Color::new(vy4, vy4, vy4), UV::new(0.0, 0.0));
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy, vz), Color::new(vy, vy, vy), UV::new(0.0, 0.0));
        }
    }
    mesh_builder.commit()
}
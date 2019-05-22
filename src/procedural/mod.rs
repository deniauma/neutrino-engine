extern crate noise;

use noise::{Perlin, NoiseFn};
use crate::graphics::mesh::{MeshBuilder, Vertex, Mesh, Color, UV, Normal};
use cgmath::prelude::*;

pub mod terrain;


pub fn heigth_map(width: usize, height: usize) -> Vec<Vec<f64>> {
    let mut map: Vec<Vec<f64>> = Vec::new();
    let noise = Noise::new();
    for x in 0 .. width {
        map.insert(x, Vec::new());
        for y in 0 .. height {
            let nx = (x as f64 / width as f64) - 0.5;
            let ny = (y as f64 / height as f64) - 0.5;
            map[x].insert(y, noise.get_elevation(nx, ny)); //map[x].insert(y, get_elevation(nx, ny));
        }
    }
    map
}



pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::new()
        }
    }

    pub fn get(&self, x: f64, z: f64) -> f64 {
        2.0 * (self.perlin.get([3.0*x, 3.0*z])/2.0 + 0.5)
    }

    pub fn get_elevation(&self, nx: f64, nz: f64) -> f64 {
        let mut e = 1.0 * self.get(1.0 * nx, 1.0 * nz) + 0.5 * self.get(2.0 * nx, 2.0 * nz) + 0.25 * self.get(4.0 * nx, 4.0 * nz);
        e /= 1.0 + 0.5 + 0.25;
        e.powf(1.4)
    }
}


pub fn generate_mesh(map: &Vec<Vec<f64>>) -> Mesh {
    let precision = 0.2;
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
            let n = calculate_normal(x, z, map);
            let n2 = calculate_normal(x + 1, z, map);
            let n3 = calculate_normal(x + 1, z + 1, map);
            let n4 = calculate_normal(x, z + 1, map);
            mesh_builder.add_vertex(vx, vy, vz).add_vertex(vx2, vy2, vz).add_vertex(vx2, vy3, vz2);
            mesh_builder.add_vertex(vx2, vy3, vz2).add_vertex(vx, vy4, vz2).add_vertex(vx, vy, vz);
            /* mesh_builder.add_color(Color::new(vy, vy, vy)).add_color(Color::new(vy2, vy2, vy2)).add_color(Color::new(vy3, vy3, vy3));
            mesh_builder.add_color(Color::new(vy3, vy3, vy3)).add_color(Color::new(vy4, vy4, vy4)).add_color(Color::new(vy, vy, vy)); */
            for _ in 0..6 {
                mesh_builder.add_color(Color::new(10.0/255.0, 112.0/255.0, 40.0/255.0));
            }
            mesh_builder.add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0));
            mesh_builder.add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0)).add_uv(UV::new(0.0, 0.0));
            mesh_builder.add_normal(n).add_normal(n2).add_normal(n3);
            mesh_builder.add_normal(n3).add_normal(n4).add_normal(n);
        }
    }
    mesh_builder.commit()
}

fn calculate_normal(x: usize, z: usize, map: &Vec<Vec<f64>>) -> Normal {
    let height_l = get_height(x as isize - 1, z as isize, map); //map[x - 1][z] as f32;
    let height_r = get_height(x as isize + 1, z as isize, map); //map[x + 1][z] as f32;
    let height_d = get_height(x as isize, z as isize - 1, map); //map[x][z - 1] as f32;
    let height_u = get_height(x as isize, z as isize + 1, map); //map[x][z + 1] as f32;
    let norm = cgmath::vec3(height_l - height_r, 2.0, height_d - height_u);
    norm.normalize()
}

fn get_height(x: isize, z: isize, map: &Vec<Vec<f64>>) -> f32 {
    if x < 0 || x > map.len() as isize - 1 || z < 0 || z > map[0].len() as isize - 1 {
        return 0.0;
    }
    map[x as usize][z as usize] as f32
}

pub fn generate_mesh_with_indices(map: &Vec<Vec<f64>>) -> Mesh {
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
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy, vz), Color::new(vy, vy, vy), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy2, vz), Color::new(vy2, vy2, vy2), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy3, vz2), Color::new(vy3, vy3, vy3), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
            mesh_builder.add_full_vertice_info(Vertex::new(vx2, vy3, vz2), Color::new(vy3, vy3, vy3), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy4, vz2), Color::new(vy4, vy4, vy4), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
            mesh_builder.add_full_vertice_info(Vertex::new(vx, vy, vz), Color::new(vy, vy, vy), UV::new(0.0, 0.0), Normal {x: 0.0, y: 1.0, z: 0.0});
        }
    }
    mesh_builder.commit()
}
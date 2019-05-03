extern crate noise;

use noise::{Perlin, NoiseFn};


pub fn heigth_map(width: usize, height: usize) -> Vec<Vec<f64>> {
    let mut map: Vec<Vec<f64>> = Vec::new();
    let perlin = Perlin::new();
    for x in 0 .. width {
        map.insert(x, Vec::new());
        for y in 0 .. height {
            let nx = (x as f64 / width as f64) - 0.5;
            let ny = (y as f64 / height as f64) - 0.5;
            map[x].insert(y, perlin.get([nx, ny]) / 2.0 + 0.5);
            // map[x][y] = perlin.get([nx, ny]);
        }
    }
    map
}
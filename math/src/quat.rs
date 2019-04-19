#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    w: f32,
    x: f32,
    y: f32,
    z: f32
}

impl Quat {
    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        let cx = (x/2.0).cos();
        let cy = (y/2.0).cos();
        let cz = (z/2.0).cos();
        let sx = (x/2.0).sin();
        let sy = (y/2.0).sin();
        let sz = (z/2.0).sin();

        Self {
            w: cx * cy * cz + sx * sy * sz,
            x: sx * cy *cz + cx * sy * sz,
            y: cx * sy * cz + sx * cy *sz,
            z: cx * cy * sz + sx * sy * cz
        }
    }
}
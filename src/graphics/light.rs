pub use cgmath::prelude::*;
pub use cgmath::{Vector3, Matrix4, Point3, Quaternion, Euler, Deg};

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>
}

impl Light {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Self {
        Self {
            position: cgmath::vec3(x, y, z),
            color: cgmath::vec3(r, g, b),
        }
    }

    pub fn default() -> Self {
        Self::new(1.0, 1.5, 2.0, 1.0, 1.0, 1.0)
    }

    pub fn rotate(&mut self, center: Vector3<f32>, angles: Vector3<f32>) {
        let rotation = Quaternion::from(Euler::new(Deg(angles.x), Deg(angles.y), Deg(angles.z)));
        self.position = rotation.rotate_vector(self.position - center);
    }
}
pub use cgmath::prelude::*;
pub use cgmath::{Vector3, Matrix4, Point3, Quaternion, Euler, Deg};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>
}

impl Camera {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Self {
            position: pos,
            direction: dir
        }
    }

    pub fn default() -> Self {
        Self {
            position: Vector3::new(0.5, 0.5, 3.0),
            direction: Vector3::new(0.0, 0.0, -1.0)
        }
    }

    pub fn lookat(&self) -> Matrix4<f32> {
        Matrix4::look_at_dir(Point3::new(self.position.x, self.position.y, self.position.z), self.direction, Vector3::new(0.0, 1.0, 0.0))
    }
}
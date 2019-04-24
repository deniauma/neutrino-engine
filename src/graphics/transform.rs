extern crate cgmath;
pub use cgmath::prelude::*;
pub use cgmath::{Vector3, Matrix4, Quaternion, Euler, Deg};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub local_transform: Matrix4<f32>,
}

impl Transform {
    pub fn new(trans: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        Self {
            translation: trans,
            rotation: rotation,
            scale: scale,
            local_transform: Matrix4::one(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            local_transform: Matrix4::one(),
        }
    }

    pub fn calculate_local_transform(&self) -> Matrix4<f32> {
        let translation_mat = Matrix4::from_translation(self.translation); //transforms::translate(Mat4::new_identity(), self.translation);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);//transforms::scale(Mat4::new_identity(), self.scale);
        let rotate_mat = Matrix4::from(Quaternion::from(Euler::new(Deg(self.rotation.x), Deg(self.rotation.y), Deg(self.rotation.z))));//self.calculate_rotation_mat();
        translation_mat * rotate_mat * scale_mat
    }

    pub fn update_local_transform(&mut self) {
        let translation_mat = Matrix4::from_translation(self.translation); //transforms::translate(Mat4::new_identity(), self.translation);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);//transforms::scale(Mat4::new_identity(), self.scale);
        let rotate_mat = Matrix4::from(Quaternion::from(Euler::new(Deg(self.rotation.x), Deg(self.rotation.y), Deg(self.rotation.z))));//self.calculate_rotation_mat();
        self.local_transform = translation_mat * rotate_mat * scale_mat;
    }

}

pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
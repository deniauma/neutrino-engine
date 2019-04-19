use math::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub local_transform: Mat4,
}

impl Transform {
    pub fn new(trans: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            translation: trans,
            rotation: rotation,
            scale: scale,
            local_transform: Mat4::new_identity(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            translation: Vec3::new_with_zeros(),
            rotation: Vec3::new_with_zeros(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            local_transform: Mat4::new_identity(),
        }
    }

    pub fn calculate_local_transform(&self) -> Mat4 {
        let translation_mat = transforms::translate(Mat4::new_identity(), self.translation);
        let scale_mat = transforms::scale(Mat4::new_identity(), self.scale);
        let rotate_mat = self.calculate_rotation_mat();
        translation_mat * rotate_mat * scale_mat
    }

    pub fn update_local_transform(&mut self) {
        let translation_mat = transforms::translate(Mat4::new_identity(), self.translation);
        let scale_mat = transforms::scale(Mat4::new_identity(), self.scale);
        println!("Scale mat: {:?}: ", scale_mat.print());
        let rotate_mat = self.calculate_rotation_mat();
        self.local_transform = translation_mat * rotate_mat * scale_mat;
    }

    fn calculate_rotation_mat(&self) -> Mat4 {
        let rotate_x_mat = transforms::get_rotate_x_mat(self.rotation.x().to_radians());
        let rotate_y_mat = transforms::get_rotate_y_mat(self.rotation.y().to_radians());
        let rotate_z_mat = transforms::get_rotate_z_mat(self.rotation.z().to_radians());
        rotate_x_mat * rotate_z_mat * rotate_y_mat
    }
}

pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
use crate::{Vec3, Mat4};

pub fn scale(mat: Mat4, scaling: Vec3) -> Mat4 {
    let scale_mat = Mat4::scaling_mat(scaling);
    scale_mat.multiply_by(mat)
}

pub fn translate(mat: Mat4, translating: Vec3) -> Mat4 {
    let translate_mat = Mat4::translate_mat(translating);
    translate_mat.multiply_by(mat)
}

pub fn rotate(mat: Mat4, axis: Vec3, angle: f32) -> Mat4 {
    let rot_mat = get_rotate_mat(axis, angle);
    rot_mat.multiply_by(mat)
}

pub fn get_rotate_mat(axis: Vec3, angle: f32) -> Mat4 {
    let rot_mat = Mat4::new_identity();
    if axis.data[0] == 1.0 {
        get_rotate_x_mat(angle)
    }
    else if axis.data[1] == 1.0 {
        get_rotate_x_mat(angle)
    }
    else if axis.data[2] == 1.0 {
        get_rotate_x_mat(angle)
    }
    else {
        rot_mat
    }
}

pub fn get_rotate_x_mat(angle: f32) -> Mat4 {
    let mut rot_mat = Mat4::new_identity();
    rot_mat.data[1][1] = angle.cos();
    rot_mat.data[2][1] = angle.sin();
    rot_mat.data[1][2] = - angle.sin();
    rot_mat.data[2][2] = angle.cos();
    return rot_mat;
}

pub fn get_rotate_y_mat(angle: f32) -> Mat4 {
    let mut rot_mat = Mat4::new_identity();
    rot_mat.data[0][0] = angle.cos();
    rot_mat.data[0][2] = angle.sin();
    rot_mat.data[2][0] = - angle.sin();
    rot_mat.data[2][2] = angle.cos();
    return rot_mat;
}

pub fn get_rotate_z_mat(angle: f32) -> Mat4 {
    let mut rot_mat = Mat4::new_identity();
    rot_mat.data[0][0] = angle.cos();
    rot_mat.data[0][1] = - angle.sin();
    rot_mat.data[1][0] = angle.sin();
    rot_mat.data[1][1] = angle.cos();
    return rot_mat;
}
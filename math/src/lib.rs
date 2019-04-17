pub mod transforms;
use std::ops::Mul;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    data: [f32; 3]
}

impl Vec3 {
    pub fn new_with_zeros() -> Self {
        Self {
            data: [0.0; 3]
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [x, y, z]
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn set_x(&mut self, x: f32) {
        self.data[0] = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.data[1] = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.data[2] = z;
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4 {
    data: [f32; 4]
}

impl Vec4 {
    pub fn new_with_zeros() -> Self {
        Self {
            data: [0.0; 4]
        }
    }

    pub fn from_vec3(vec: Vec3, w: f32) -> Self {
        Self {
            data: [vec.x(), vec.y(), vec.z(), w]
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4]
}

impl Mat4 {
    pub fn new_with_zeros() -> Self {
        Self {
            data: [[0.0; 4]; 4]
        }
    }

    pub fn new_identity() -> Self {
        let mut mat = Mat4::new_with_zeros();
        for row in  0..4 {
            for col in 0..4 {
                if col == row {
                    mat.data[row][col] = 1.0;
                }
                
            }
        }
        mat
    }

    pub fn multiply_by(&self, mat: Self) -> Self {
        let mut result = Mat4::new_with_zeros();
        //result.data[0][0] = self.data[0][0] * mat.data[0][0] + self.data[0][1] * mat.data[1][0] + self.data[0][2] * mat.data[2][0] + self.data[0][3] * mat.data[3][0];
        for row in  0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    result.data[row][col] += self.data[row][i] * mat.data[i][col];
                }
            }
        }
        result
    }

    pub fn multiply_by_vec(&self, vec: Vec4) -> Vec4 {
        let mut result = Vec4::new_with_zeros();
        for row in  0..4 {
            for i in 0..4 {
                result.data[row] += self.data[row][i] * vec.data[i];
            }
        }
        result
    }

    pub fn scaling_mat(scale: Vec3) -> Self {
        let mut mat = Mat4::new_identity();
        mat.data[0][0] = scale.data[0];
        mat.data[1][1] = scale.data[1];
        mat.data[2][2] = scale.data[2];
        mat
    }

    pub fn translate_mat(translate: Vec3) -> Self {
        let mut mat = Mat4::new_identity();
        mat.data[0][3] = translate.data[0];
        mat.data[1][3] = translate.data[1];
        mat.data[2][3] = translate.data[2];
        mat
    }  

    pub fn print(&self) {
        for row in  0..4 {
            let mut line = String::new();
            for col in 0..4 {
                line.push_str(format!("{:} ", self.data[row][col]).as_str());
            }
            println!("{}", line);
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        self.multiply_by(other)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        self.multiply_by_vec(other)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat4_zeros() {
        let mat = Mat4::new_with_zeros();
        let mat2 = Mat4 { data: [[0.0;4]; 4] };
        assert_eq!(mat, mat2);
    }

    #[test]
    fn mat4_identity() {
        let mat = Mat4::new_identity();
        mat.print();
        let mat2 = Mat4 { data: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        assert_eq!(mat, mat2);
    }

    #[test]
    fn mat4_multiplication() {
        let mat1 = Mat4::new_with_zeros();
        let mat2 = Mat4::new_identity();
        let mat3 = mat1.multiply_by(mat2);
        assert_eq!(mat1, mat3);
    }

    #[test]
    fn mat4_mul_opt() {
        let mat1 = Mat4::new_with_zeros();
        let mat2 = Mat4::new_identity();
        let mat3 = mat1 * mat2;
        assert_eq!(mat1, mat3);
    }

    #[test]
    fn mat4_multiply_by_vec4() {
        let mat = Mat4::new_identity();
        let vec1 = Vec4{ data: [2.0; 4] };
        let vec2 = mat.multiply_by_vec(vec1);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn mat4_mul_by_vec4_opt() {
        let mat = Mat4::new_identity();
        let vec1 = Vec4{ data: [2.0; 4] };
        let vec2 = mat * vec1;
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn mat4_scale() {
        let vec = Vec3{ data: [1.0, 2.0, 3.0] };
        let mat = Mat4::scaling_mat(vec);
        let mut mat2 = Mat4::new_with_zeros();
        mat2.data[0][0] = 1.0;
        mat2.data[1][1] = 2.0;
        mat2.data[2][2] = 3.0;
        mat2.data[3][3] = 1.0;
        assert_eq!(mat, mat2);
    }
}

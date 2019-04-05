#[derive(Debug, PartialEq)]
pub struct Mat4 {
    data: [[f32; 4]; 4]
}

impl Mat4 {
    pub fn new_with_zeros() -> Self {
        Self {
            data: [[0.0; 4]; 4]
        }
    }

    pub fn identity(&mut self) {
        for row in  0..4 {
            for col in 0..4 {
                if col == row {
                    self.data[row][col] = 1.0;
                }
                
            }
        }
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
        let mut mat = Mat4::new_with_zeros();
        mat.identity();
        mat.print();
        let mat2 = Mat4 { data: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
        assert_eq!(mat, mat2);
    }
}

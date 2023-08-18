use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    data: [[f64; 2]; 2],
}

impl Index<[usize; 2]> for Matrix2 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix2 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}

impl Matrix2 {
    pub fn create_and_fill(fill_value: f64) -> Matrix2 {
        Matrix2 {
            data: [[fill_value; 2]; 2],
        }
    }

    pub fn zeros() -> Matrix2 {
        Matrix2::create_and_fill(0.0)
    }

    pub fn determinant(&self) -> f64 {
        self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{float_eq, FLOAT_EQ_EPS};

    use super::*;

    #[test]
    fn determinant() {
        let mut mat = Matrix2::zeros();
        mat[[0, 0]] = 1.0;
        mat[[1, 0]] = 5.0;
        mat[[0, 1]] = -3.0;
        mat[[1, 1]] = 2.0;
        assert!(float_eq(mat.determinant(), 17.0, FLOAT_EQ_EPS))
    }
}

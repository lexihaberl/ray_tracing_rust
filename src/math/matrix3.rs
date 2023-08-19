use super::{float_eq, Matrix2, FLOAT_EQ_EPS};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

impl Index<[usize; 2]> for Matrix3 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix3 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..=2 {
            for j in 0..=2 {
                if !float_eq(self[[i, j]], other[[i, j]], FLOAT_EQ_EPS) {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix3 {
    pub fn create_and_fill(fill_value: f64) -> Matrix3 {
        Matrix3 {
            data: [[fill_value; 3]; 3],
        }
    }

    pub fn zeros() -> Matrix3 {
        Matrix3::create_and_fill(0.0)
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut sub_matr = Matrix2::zeros();
        let mut new_i = 0;
        let mut new_j = 0;
        for i in 0..=2 {
            if i == row {
                continue;
            }
            for j in 0..=2 {
                if j == col {
                    continue;
                }
                sub_matr[[new_i, new_j]] = self[[i, j]];
                new_j += 1;
            }
            new_i += 1;
            new_j = 0;
        }
        sub_matr
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 != 0 {
            return -minor;
        }
        minor
    }

    pub fn determinant(&self) -> f64 {
        self.cofactor(0, 0) * self[[0, 0]]
            + self.cofactor(0, 1) * self[[0, 1]]
            + self.cofactor(0, 2) * self[[0, 2]]
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix2;

    use super::*;

    #[test]
    fn matrix_equality() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[1, 0]] = 2.0;
        matrix[[2, 0]] = 3.0;
        matrix[[0, 1]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[2, 1]] = 7.0;
        matrix[[0, 2]] = 9.0;
        matrix[[1, 2]] = 8.0;
        matrix[[2, 2]] = 7.0;
        // matrix2 is a copy, not a reference
        let matrix2 = matrix;
        matrix[[0, 0]] = 1.0 + 0.000000001;
        assert_eq!(matrix, matrix2)
    }

    #[test]
    fn matrix_inequality() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[1, 0]] = 2.0;
        matrix[[2, 0]] = 3.0;
        matrix[[0, 1]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[2, 1]] = 7.0;
        matrix[[0, 2]] = 9.0;
        matrix[[1, 2]] = 8.0;
        matrix[[2, 2]] = 7.0;
        // matrix2 is a copy, not a reference
        let matrix2 = matrix;
        matrix[[2, 2]] = 3.0;
        assert_ne!(matrix, matrix2);
    }

    #[test]
    fn submatrix() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[1, 0]] = 2.0;
        matrix[[2, 0]] = 3.0;
        matrix[[0, 1]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[2, 1]] = 7.0;
        matrix[[0, 2]] = 9.0;
        matrix[[1, 2]] = 8.0;
        matrix[[2, 2]] = 7.0;
        // matrix2 is a copy, not a reference
        let mut expected_submatrix = Matrix2::zeros();
        expected_submatrix[[0, 0]] = 5.0;
        expected_submatrix[[1, 0]] = 6.0;
        expected_submatrix[[0, 1]] = 9.0;
        expected_submatrix[[1, 1]] = 8.0;
        assert_eq!(matrix.submatrix(2, 0), expected_submatrix)
    }

    #[test]
    fn minor() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 3.0;
        matrix[[0, 1]] = 5.0;
        matrix[[0, 2]] = 0.0;
        matrix[[1, 0]] = 2.0;
        matrix[[1, 1]] = -1.0;
        matrix[[1, 2]] = -7.0;
        matrix[[2, 0]] = 6.0;
        matrix[[2, 1]] = -1.0;
        matrix[[2, 2]] = 5.0;

        assert!(float_eq(matrix.minor(1, 0), 25.0, FLOAT_EQ_EPS))
    }

    #[test]
    fn minor_and_cofactors() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 3.0;
        matrix[[0, 1]] = 5.0;
        matrix[[0, 2]] = 0.0;
        matrix[[1, 0]] = 2.0;
        matrix[[1, 1]] = -1.0;
        matrix[[1, 2]] = -7.0;
        matrix[[2, 0]] = 6.0;
        matrix[[2, 1]] = -1.0;
        matrix[[2, 2]] = 5.0;

        assert!(float_eq(matrix.minor(1, 0), 25.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(1, 0), -25.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.minor(0, 0), -12.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 0), -12.0, FLOAT_EQ_EPS));
    }

    #[test]
    fn determinant() {
        let mut matrix = Matrix3::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[0, 1]] = 2.0;
        matrix[[0, 2]] = 6.0;
        matrix[[1, 0]] = -5.0;
        matrix[[1, 1]] = 8.0;
        matrix[[1, 2]] = -4.0;
        matrix[[2, 0]] = 2.0;
        matrix[[2, 1]] = 6.0;
        matrix[[2, 2]] = 4.0;

        assert!(float_eq(matrix.cofactor(0, 0), 56.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 1), 12.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 2), -46.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.determinant(), -196.0, FLOAT_EQ_EPS));
    }
}

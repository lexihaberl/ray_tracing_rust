use std::ops::{Index, IndexMut, Mul};

use super::{float_eq, Matrix3, Tuple4D, FLOAT_EQ_EPS};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    data: [[f64; 4]; 4],
}

impl Index<[usize; 2]> for Matrix4 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix4 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..=3 {
            for j in 0..=3 {
                if !float_eq(self[[i, j]], other[[i, j]], FLOAT_EQ_EPS) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix4::zeros();
        for i in 0..=3 {
            for j in 0..=3 {
                for k in 0..=3 {
                    res[[i, j]] += self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        res
    }
}

impl Mul<Tuple4D> for Matrix4 {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Self::Output {
        let mut res = Tuple4D::zeros();
        for i in 0..=3 {
            res[i] = self[[i, 0]] * rhs[0]
                + self[[i, 1]] * rhs[1]
                + self[[i, 2]] * rhs[2]
                + self[[i, 3]] * rhs[3];
        }
        res
    }
}

impl Matrix4 {
    pub fn create_and_fill(fill_value: f64) -> Matrix4 {
        Matrix4 {
            data: [[fill_value; 4]; 4],
        }
    }
    pub fn zeros() -> Matrix4 {
        Matrix4::create_and_fill(0.0)
    }

    pub fn eye() -> Matrix4 {
        let mut mat = Matrix4::create_and_fill(0.0);
        mat[[0, 0]] = 1.0;
        mat[[1, 1]] = 1.0;
        mat[[2, 2]] = 1.0;
        mat[[3, 3]] = 1.0;
        mat
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut transposed_matrix = Matrix4::zeros();
        for i in 0..=3 {
            for j in 0..=3 {
                transposed_matrix[[i, j]] = self[[j, i]]
            }
        }
        transposed_matrix
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut sub_matr = Matrix3::zeros();
        let mut new_i = 0;
        let mut new_j = 0;
        for i in 0..=3 {
            if i == row {
                continue;
            }
            for j in 0..=3 {
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
            + self.cofactor(0, 3) * self[[0, 3]]
    }

    pub fn inverse(&self) -> Option<Matrix4> {
        let det = self.determinant();
        if float_eq(det, 0.0, FLOAT_EQ_EPS) {
            return None;
        }

        let mut inverse = Matrix4::zeros();
        for i in 0..=3 {
            for j in 0..=3 {
                let cofactor = self.cofactor(i, j);

                inverse[[j, i]] = cofactor / det;
            }
        }
        Option::Some(inverse)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{matrix3, Tuple4D};

    use super::*;

    #[test]
    fn matrix_equality() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[1, 0]] = 2.0;
        matrix[[2, 0]] = 3.0;
        matrix[[3, 0]] = 4.0;
        matrix[[0, 1]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[2, 1]] = 7.0;
        matrix[[3, 1]] = 8.0;
        matrix[[0, 2]] = 9.0;
        matrix[[1, 2]] = 8.0;
        matrix[[2, 2]] = 7.0;
        matrix[[3, 2]] = 6.0;
        matrix[[0, 3]] = 5.0;
        matrix[[1, 3]] = 4.0;
        matrix[[2, 3]] = 3.0;
        matrix[[3, 3]] = 2.0;
        // matrix2 is a copy, not a reference
        let matrix2 = matrix;
        matrix[[0, 0]] = 1.0 + 0.000000001;
        assert_eq!(matrix, matrix2)
    }

    #[test]
    fn matrix_inequality() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[1, 0]] = 2.0;
        matrix[[2, 0]] = 3.0;
        matrix[[3, 0]] = 4.0;
        matrix[[0, 1]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[2, 1]] = 7.0;
        matrix[[3, 1]] = 8.0;
        matrix[[0, 2]] = 9.0;
        matrix[[1, 2]] = 8.0;
        matrix[[2, 2]] = 7.0;
        matrix[[3, 2]] = 6.0;
        matrix[[0, 3]] = 5.0;
        matrix[[1, 3]] = 4.0;
        matrix[[2, 3]] = 3.0;
        matrix[[3, 3]] = 2.0;
        // matrix2 is a copy, not a reference
        let matrix2 = matrix;
        matrix[[3, 3]] = 3.0;
        assert_ne!(matrix, matrix2);
    }

    #[test]
    fn mult_with_identity() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -2.0;
        matrix[[0, 1]] = 1.0;
        matrix[[0, 2]] = 2.0;
        matrix[[0, 3]] = 3.0;
        matrix[[1, 0]] = 3.0;
        matrix[[1, 1]] = 2.0;
        matrix[[1, 2]] = 1.0;
        matrix[[1, 3]] = -1.0;
        matrix[[2, 0]] = 4.0;
        matrix[[2, 1]] = 3.0;
        matrix[[2, 2]] = 6.0;
        matrix[[2, 3]] = 5.0;
        matrix[[3, 0]] = 1.0;
        matrix[[3, 1]] = 2.0;
        matrix[[3, 2]] = 7.0;
        matrix[[3, 3]] = 8.0;

        assert_eq!(matrix * Matrix4::eye(), matrix)
    }

    #[test]
    fn mult_tupl_with_identity() {
        let tuple = Tuple4D::new_point(1.0, 2.0, 30.0);
        assert_eq!(Matrix4::eye() * tuple, tuple);
    }

    #[test]
    fn matrix_mult() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[0, 1]] = 2.0;
        matrix[[0, 2]] = 3.0;
        matrix[[0, 3]] = 4.0;
        matrix[[1, 0]] = 5.0;
        matrix[[1, 1]] = 6.0;
        matrix[[1, 2]] = 7.0;
        matrix[[1, 3]] = 8.0;
        matrix[[2, 0]] = 9.0;
        matrix[[2, 1]] = 8.0;
        matrix[[2, 2]] = 7.0;
        matrix[[2, 3]] = 6.0;
        matrix[[3, 0]] = 5.0;
        matrix[[3, 1]] = 4.0;
        matrix[[3, 2]] = 3.0;
        matrix[[3, 3]] = 2.0;

        let mut matrix2 = Matrix4::zeros();
        matrix2[[0, 0]] = -2.0;
        matrix2[[0, 1]] = 1.0;
        matrix2[[0, 2]] = 2.0;
        matrix2[[0, 3]] = 3.0;
        matrix2[[1, 0]] = 3.0;
        matrix2[[1, 1]] = 2.0;
        matrix2[[1, 2]] = 1.0;
        matrix2[[1, 3]] = -1.0;
        matrix2[[2, 0]] = 4.0;
        matrix2[[2, 1]] = 3.0;
        matrix2[[2, 2]] = 6.0;
        matrix2[[2, 3]] = 5.0;
        matrix2[[3, 0]] = 1.0;
        matrix2[[3, 1]] = 2.0;
        matrix2[[3, 2]] = 7.0;
        matrix2[[3, 3]] = 8.0;

        let mut matrix_expected = Matrix4::zeros();
        matrix_expected[[0, 0]] = 20.0;
        matrix_expected[[0, 1]] = 22.0;
        matrix_expected[[0, 2]] = 50.0;
        matrix_expected[[0, 3]] = 48.0;
        matrix_expected[[1, 0]] = 44.0;
        matrix_expected[[1, 1]] = 54.0;
        matrix_expected[[1, 2]] = 114.0;
        matrix_expected[[1, 3]] = 108.0;
        matrix_expected[[2, 0]] = 40.0;
        matrix_expected[[2, 1]] = 58.0;
        matrix_expected[[2, 2]] = 110.0;
        matrix_expected[[2, 3]] = 102.0;
        matrix_expected[[3, 0]] = 16.0;
        matrix_expected[[3, 1]] = 26.0;
        matrix_expected[[3, 2]] = 46.0;
        matrix_expected[[3, 3]] = 42.0;

        assert_eq!(matrix * matrix2, matrix_expected)
    }

    #[test]
    fn matrix_tuple_product() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = 1.0;
        matrix[[0, 1]] = 2.0;
        matrix[[0, 2]] = 3.0;
        matrix[[0, 3]] = 4.0;
        matrix[[1, 0]] = 2.0;
        matrix[[1, 1]] = 4.0;
        matrix[[1, 2]] = 4.0;
        matrix[[1, 3]] = 2.0;
        matrix[[2, 0]] = 8.0;
        matrix[[2, 1]] = 6.0;
        matrix[[2, 2]] = 4.0;
        matrix[[2, 3]] = 1.0;
        matrix[[3, 0]] = 0.0;
        matrix[[3, 1]] = 0.0;
        matrix[[3, 2]] = 0.0;
        matrix[[3, 3]] = 1.0;

        let point = Tuple4D::new_point(1.0, 2.0, 3.0);

        assert_eq!(matrix * point, Tuple4D::new_point(18., 24., 33.))
    }

    #[test]
    fn transposition() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -2.0;
        matrix[[0, 1]] = 1.0;
        matrix[[0, 2]] = 2.0;
        matrix[[0, 3]] = 3.0;
        matrix[[1, 0]] = 3.0;
        matrix[[1, 1]] = 2.0;
        matrix[[1, 2]] = 1.0;
        matrix[[1, 3]] = -1.0;
        matrix[[2, 0]] = 4.0;
        matrix[[2, 1]] = 3.0;
        matrix[[2, 2]] = 6.0;
        matrix[[2, 3]] = 5.0;
        matrix[[3, 0]] = 1.0;
        matrix[[3, 1]] = 2.0;
        matrix[[3, 2]] = 7.0;
        matrix[[3, 3]] = 8.0;

        let mut expected = Matrix4::zeros();
        expected[[0, 0]] = -2.0;
        expected[[1, 0]] = 1.0;
        expected[[2, 0]] = 2.0;
        expected[[3, 0]] = 3.0;
        expected[[0, 1]] = 3.0;
        expected[[1, 1]] = 2.0;
        expected[[2, 1]] = 1.0;
        expected[[3, 1]] = -1.0;
        expected[[0, 2]] = 4.0;
        expected[[1, 2]] = 3.0;
        expected[[2, 2]] = 6.0;
        expected[[3, 2]] = 5.0;
        expected[[0, 3]] = 1.0;
        expected[[1, 3]] = 2.0;
        expected[[2, 3]] = 7.0;
        expected[[3, 3]] = 8.0;

        assert_eq!(matrix.transpose(), expected)
    }

    #[test]
    fn submatrix() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -2.0;
        matrix[[0, 1]] = 1.0;
        matrix[[0, 2]] = 2.0;
        matrix[[0, 3]] = 3.0;
        matrix[[1, 0]] = 3.0;
        matrix[[1, 1]] = 2.0;
        matrix[[1, 2]] = 1.0;
        matrix[[1, 3]] = -1.0;
        matrix[[2, 0]] = 4.0;
        matrix[[2, 1]] = 3.0;
        matrix[[2, 2]] = 6.0;
        matrix[[2, 3]] = 5.0;
        matrix[[3, 0]] = 1.0;
        matrix[[3, 1]] = 2.0;
        matrix[[3, 2]] = 7.0;
        matrix[[3, 3]] = 8.0;

        let mut expected_submatrix = Matrix3::zeros();
        expected_submatrix[[0, 0]] = -2.0;
        expected_submatrix[[0, 1]] = 2.0;
        expected_submatrix[[0, 2]] = 3.0;
        expected_submatrix[[1, 0]] = 3.0;
        expected_submatrix[[1, 1]] = 1.0;
        expected_submatrix[[1, 2]] = -1.0;
        expected_submatrix[[2, 0]] = 4.0;
        expected_submatrix[[2, 1]] = 6.0;
        expected_submatrix[[2, 2]] = 5.0;
        assert_eq!(matrix.submatrix(3, 1), expected_submatrix);
    }

    #[test]
    fn determinant() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -2.0;
        matrix[[0, 1]] = -8.0;
        matrix[[0, 2]] = 3.0;
        matrix[[0, 3]] = 5.0;
        matrix[[1, 0]] = -3.0;
        matrix[[1, 1]] = 1.0;
        matrix[[1, 2]] = 7.0;
        matrix[[1, 3]] = 3.0;
        matrix[[2, 0]] = 1.0;
        matrix[[2, 1]] = 2.0;
        matrix[[2, 2]] = -9.0;
        matrix[[2, 3]] = 6.0;
        matrix[[3, 0]] = -6.0;
        matrix[[3, 1]] = 7.0;
        matrix[[3, 2]] = 7.0;
        matrix[[3, 3]] = -9.0;
        println!("{}", matrix.cofactor(0, 1));

        assert!(float_eq(matrix.cofactor(0, 0), 690.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 1), 447.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 2), 210.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(0, 3), 51.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.determinant(), -4071.0, FLOAT_EQ_EPS));
    }

    #[test]
    fn non_invertible() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -4.0;
        matrix[[0, 1]] = 2.0;
        matrix[[0, 2]] = -2.0;
        matrix[[0, 3]] = -3.0;
        matrix[[1, 0]] = 9.0;
        matrix[[1, 1]] = 6.0;
        matrix[[1, 2]] = 2.0;
        matrix[[1, 3]] = 6.0;
        matrix[[2, 0]] = 0.0;
        matrix[[2, 1]] = -5.0;
        matrix[[2, 2]] = 1.0;
        matrix[[2, 3]] = -5.0;
        matrix[[3, 0]] = 0.0;
        matrix[[3, 1]] = 0.0;
        matrix[[3, 2]] = 0.0;
        matrix[[3, 3]] = 0.0;

        assert_eq!(matrix.inverse(), None)
    }

    #[test]
    fn inverse() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = -5.0;
        matrix[[0, 1]] = 2.0;
        matrix[[0, 2]] = 6.0;
        matrix[[0, 3]] = -8.0;
        matrix[[1, 0]] = 1.0;
        matrix[[1, 1]] = -5.0;
        matrix[[1, 2]] = 1.0;
        matrix[[1, 3]] = 8.0;
        matrix[[2, 0]] = 7.0;
        matrix[[2, 1]] = 7.0;
        matrix[[2, 2]] = -6.0;
        matrix[[2, 3]] = -7.0;
        matrix[[3, 0]] = 1.0;
        matrix[[3, 1]] = -3.0;
        matrix[[3, 2]] = 7.0;
        matrix[[3, 3]] = 4.0;

        assert!(float_eq(matrix.determinant(), 532.0, FLOAT_EQ_EPS));
        assert!(float_eq(matrix.cofactor(2, 3), -160.0, FLOAT_EQ_EPS));

        let mut expected_inv = Matrix4::zeros();
        expected_inv[[0, 0]] = 0.21805;
        expected_inv[[0, 1]] = 0.45113;
        expected_inv[[0, 2]] = 0.24060;
        expected_inv[[0, 3]] = -0.04511;
        expected_inv[[1, 0]] = -0.80827;
        expected_inv[[1, 1]] = -1.45677;
        expected_inv[[1, 2]] = -0.44361;
        expected_inv[[1, 3]] = 0.52068;
        expected_inv[[2, 0]] = -0.07895;
        expected_inv[[2, 1]] = -0.22368;
        expected_inv[[2, 2]] = -0.05263;
        expected_inv[[2, 3]] = 0.19737;
        expected_inv[[3, 0]] = -0.52256;
        expected_inv[[3, 1]] = -0.81391;
        expected_inv[[3, 2]] = -0.30075;
        expected_inv[[3, 3]] = 0.30639;
        assert_eq!(matrix.inverse().unwrap(), expected_inv);
    }

    #[test]
    fn inverse_multiplication() {
        let mut matrix = Matrix4::zeros();
        matrix[[0, 0]] = 3.0;
        matrix[[0, 1]] = -9.0;
        matrix[[0, 2]] = 7.0;
        matrix[[0, 3]] = 3.0;
        matrix[[1, 0]] = 3.0;
        matrix[[1, 1]] = -8.0;
        matrix[[1, 2]] = 2.0;
        matrix[[1, 3]] = -9.0;
        matrix[[2, 0]] = -4.0;
        matrix[[2, 1]] = 4.0;
        matrix[[2, 2]] = 4.0;
        matrix[[2, 3]] = 1.0;
        matrix[[3, 0]] = -6.0;
        matrix[[3, 1]] = 5.0;
        matrix[[3, 2]] = -1.0;
        matrix[[3, 3]] = 1.0;

        let mut matrix2 = Matrix4::zeros();
        matrix2[[0, 0]] = -2.0;
        matrix2[[0, 1]] = 1.0;
        matrix2[[0, 2]] = 2.0;
        matrix2[[0, 3]] = 3.0;
        matrix2[[1, 0]] = 3.0;
        matrix2[[1, 1]] = 2.0;
        matrix2[[1, 2]] = 1.0;
        matrix2[[1, 3]] = -1.0;
        matrix2[[2, 0]] = 4.0;
        matrix2[[2, 1]] = 3.0;
        matrix2[[2, 2]] = 6.0;
        matrix2[[2, 3]] = 5.0;
        matrix2[[3, 0]] = 1.0;
        matrix2[[3, 1]] = 2.0;
        matrix2[[3, 2]] = 7.0;
        matrix2[[3, 3]] = 8.0;

        let matrix3 = matrix * matrix2;
        assert_eq!(matrix, matrix3 * matrix2.inverse().unwrap())
    }
}

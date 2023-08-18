use std::ops::{Index, IndexMut, Mul};

use super::{float_eq, Tuple4D, FLOAT_EQ_EPS};

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
}

#[cfg(test)]
mod tests {
    use crate::math::Tuple4D;

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
}

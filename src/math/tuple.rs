use crate::math::{float_eq, FLOAT_EQ_EPS};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Tuple4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple4D {
    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple4D {
        Tuple4D { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple4D {
        Tuple4D { x, y, z, w: 0.0 }
    }

    pub fn is_vector(self) -> bool {
        float_eq(self.w, 0.0, FLOAT_EQ_EPS)
    }

    pub fn is_point(self) -> bool {
        float_eq(self.w, 1.0, FLOAT_EQ_EPS)
    }

    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, other: Self) -> f64 {
        if !self.is_vector() || !other.is_vector() {
            panic!("Called dot product on a tuple that is not a vector");
        }
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        if !self.is_vector() || !other.is_vector() {
            panic!("Called cross product on a tuple that is not a vector");
        }
        Self::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple4D {
    fn eq(&self, other: &Self) -> bool {
        let eps = FLOAT_EQ_EPS;
        float_eq(self.x, other.x, eps)
            && float_eq(self.y, other.y, eps)
            && float_eq(self.z, other.z, eps)
            && float_eq(self.w, other.w, eps)
    }
}

impl Add for Tuple4D {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple4D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple4D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple4D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple4D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple4D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_is_not_point() {
        let vec = Tuple4D::new_vector(1.0, 3.0, 7.0);
        assert!(!vec.is_point());
        let vec = Tuple4D::new_vector(2.0, 6.0, 11.0);
        assert!(!vec.is_point());
        let vec = Tuple4D::new_vector(1.0, 3.0, 7.0);
        assert!(!vec.is_point());
    }

    #[test]
    fn vector_is_vector() {
        let vec = Tuple4D::new_vector(1.0, 3.0, 7.0);
        assert!(vec.is_vector());
        let vec = Tuple4D::new_vector(2.0, 6.0, 11.0);
        assert!(vec.is_vector());
        let vec = Tuple4D::new_vector(-1.0, 3.0, -7.0);
        assert!(vec.is_vector());
    }

    #[test]
    fn vector_has_zero_valued_w() {
        let vec = Tuple4D::new_vector(1.0, 3.0, 7.0);
        assert!(float_eq(vec.w, 0.0, FLOAT_EQ_EPS));
    }

    #[test]
    fn point_is_not_vector() {
        let point = Tuple4D::new_point(1.0, 3.0, 7.0);
        assert!(!point.is_vector());
        let point = Tuple4D::new_point(2.0, 6.0, 11.0);
        assert!(!point.is_vector());
        let point = Tuple4D::new_point(-1.0, 3.0, -7.0);
        assert!(!point.is_vector());
    }

    #[test]
    fn point_is_point() {
        let point = Tuple4D::new_point(1.0, 3.0, 7.0);
        assert!(point.is_point());
        let point = Tuple4D::new_point(2.0, 6.0, 11.0);
        assert!(point.is_point());
        let point = Tuple4D::new_point(-1.0, 3.0, -7.0);
        assert!(point.is_point());
    }

    #[test]
    fn point_has_w_value_of_one() {
        let point = Tuple4D::new_point(1.0, 3.0, 7.0);
        assert!(float_eq(point.w, 1.0, FLOAT_EQ_EPS));
    }

    #[test]
    fn tuple_4d_equal_test() {
        let a = Tuple4D::new_point(0.5 + 0.3 + 0.2, 0.2 + 0.1 - 0.4, -0.6 - 0.3);
        let b = Tuple4D::new_point(0.4 + 0.3 + 0.2 + 0.1, -0.1, -0.3 - 0.3 - 0.3);
        assert_eq!(a, b);
    }

    #[test]
    fn tuple_4d_unequal_test() {
        let a = Tuple4D::new_point(0.5 + 0.3 + 0.2, 0.2 + 0.1 - 0.4, -0.6 - 0.3);
        let b = Tuple4D::new_point(0.4 + 0.3 + 0.2 + 0.1, -0.1, -0.3);
        assert_ne!(a, b);
    }

    #[test]
    fn add_vector_to_point_is_point() {
        let a = Tuple4D::new_point(3.0, -2.0, 5.0);
        let b = Tuple4D::new_vector(-2.0, 3.0, 1.0);
        assert_eq!(a + b, Tuple4D::new_point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_point_from_point_is_vector() {
        let a = Tuple4D::new_point(3.0, 2.0, 1.0);
        let b = Tuple4D::new_point(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple4D::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point_is_point() {
        let pt_a = Tuple4D::new_point(3.0, 2.0, 1.0);
        let vec_b = Tuple4D::new_vector(5.0, 6.0, 7.0);
        assert_eq!(pt_a - vec_b, Tuple4D::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors_is_vector() {
        let vec_a = Tuple4D::new_vector(3.0, 2.0, 1.0);
        let vec_b = Tuple4D::new_vector(5.0, 6.0, 7.0);
        assert_eq!(vec_a - vec_b, Tuple4D::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple4D::new_vector(1.0, -2.0, 3.0);
        assert_eq!(-a, Tuple4D::new_vector(-1.0, 2.0, -3.0));

        let a = Tuple4D::new_point(1.0, -2.0, 3.0);
        assert_eq!(
            -a,
            Tuple4D {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: -1.0
            }
        );
    }

    #[test]
    fn multiplying_tuple_by_scalars() {
        let a = Tuple4D::new_point(1.0, -2.0, 3.0);
        assert_eq!(
            a * 3.5,
            Tuple4D {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: 3.5
            }
        );

        let b = Tuple4D::new_point(1.0, -2.0, 3.0);
        assert_eq!(
            b * 0.5,
            Tuple4D {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: 0.5
            }
        );
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple4D::new_point(1.0, -2.0, 3.0);
        assert_eq!(
            a / 2.0,
            Tuple4D {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: 0.5
            }
        );
    }

    #[test]
    fn check_magnitude() {
        let a = Tuple4D::new_vector(0.0, 1.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);
        let b = Tuple4D::new_vector(0.0, 0.0, 1.0);
        assert_eq!(b.magnitude(), 1.0);
        let c = Tuple4D::new_vector(2.0, 3.0, 4.0);
        assert_eq!(c.magnitude(), (29.0_f64).sqrt());
        let d = Tuple4D::new_point(1.0, 1.0, 1.0);
        assert_eq!(d.magnitude(), 2.0);
    }

    #[test]
    fn check_normalizations() {
        let a = Tuple4D::new_vector(4.0, 0.0, 0.0);
        assert_eq!(a.normalize(), Tuple4D::new_vector(1.0, 0.0, 0.0));
        assert_eq!(a.normalize().magnitude(), 1.0);

        let b = Tuple4D::new_vector(1.0, 2.0, 3.0);
        assert_eq!(
            b.normalize(),
            Tuple4D::new_vector(
                1.0 / (14.0_f64).sqrt(),
                2.0 / (14.0_f64).sqrt(),
                3.0 / (14.0_f64).sqrt()
            )
        );
        assert_eq!(b.normalize().magnitude(), 1.0);
    }

    #[test]
    fn check_dot_product() {
        let a = Tuple4D::new_vector(1.0, 2.0, 3.0);
        let b = Tuple4D::new_vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    #[should_panic]
    fn dot_product_on_point() {
        let a = Tuple4D::new_point(1.0, 2.0, 3.0);
        a.dot(a);
    }

    #[test]
    fn check_cross_product() {
        let a = Tuple4D::new_vector(1.0, 2.0, 3.0);
        let b = Tuple4D::new_vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple4D::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple4D::new_vector(1.0, -2.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn cross_product_on_point() {
        let a = Tuple4D::new_point(1.0, 2.0, 3.0);
        a.cross(a);
    }
}

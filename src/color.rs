use crate::math::{float_eq, FLOAT_EQ_EPS};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    fn mul_color(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let eps = FLOAT_EQ_EPS;
        float_eq(self.r, other.r, eps)
            && float_eq(self.g, other.g, eps)
            && float_eq(self.b, other.b, eps)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_init() {
        let col = Color::new(-2.0, 1.0, 0.4);
        let col2 = Color {
            r: -2.0,
            g: 1.0,
            b: 0.4,
        };
        assert_eq!(col, col2);
    }

    #[test]
    fn colors_adding() {
        let col1 = Color::new(0.9, 0.6, 0.75);
        let col2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(col1 + col2, Color::new(1.6, 0.7, 1.0))
    }

    #[test]
    fn colors_subtracting() {
        let col1 = Color::new(0.9, 0.6, 0.75);
        let col2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(col1 - col2, Color::new(0.2, 0.5, 0.5))
    }

    #[test]
    fn color_scalar_mult() {
        let col = Color::new(0.2, 0.5, 0.5);
        assert_eq!(col * 2.0, Color::new(0.4, 1.0, 1.0))
    }

    #[test]
    fn color_color_mult() {
        let col = Color::new(1.0, 0.2, 0.4);
        let col2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(col.mul_color(col2), Color::new(0.9, 0.2, 0.04))
    }
}

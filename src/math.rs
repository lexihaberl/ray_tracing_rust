mod tuple;
pub use tuple::Tuple4D;

pub fn float_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}
pub const FLOAT_EQ_EPS: f64 = 0.00001;

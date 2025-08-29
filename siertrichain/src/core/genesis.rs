
use crate::core::geometry::Point;
use crate::core::triangle::Triangle;

use rust_decimal_macros::dec;
use rust_decimal::MathematicalOps;

pub struct GenesisTriangle;

impl GenesisTriangle {
    pub fn new() -> Triangle {
        let a = Point::new(dec!(0), dec!(0));
        let b = Point::new(dec!(1), dec!(0));
        let y = (dec!(3).sqrt().unwrap()) / dec!(2);
        let c = Point::new(dec!(0.5), y);
        Triangle::new(a, b, c)
    }

    pub fn validate(triangle: &Triangle) -> bool {
        let side1 = triangle.a.distance(&triangle.b);
        let side2 = triangle.b.distance(&triangle.c);
        let side3 = triangle.c.distance(&triangle.a);

        let is_equilateral = (side1 - side2).abs() < dec!(1e-9) && (side2 - side3).abs() < dec!(1e-9);
        let is_perfect_area = (triangle.area() - (dec!(3).sqrt().unwrap() / dec!(4))).abs() < dec!(1e-9);

        is_equilateral && is_perfect_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_triangle_creation() {
        let genesis = GenesisTriangle::new();
        assert!(GenesisTriangle::validate(&genesis));
    }
}

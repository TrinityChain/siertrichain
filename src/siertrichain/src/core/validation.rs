
use crate::core::fractal::FractalTriangle;
use crate::core::genesis::GenesisTriangle;
use crate::core::triangle::Triangle;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn validate_triangle(triangle: &Triangle) -> bool {
    triangle.is_valid()
}

pub fn validate_fractal_triangle(fractal_tri: &FractalTriangle, parent_tri: Option<&FractalTriangle>) -> bool {
    if let Some(parent) = parent_tri {
        if fractal_tri.depth != parent.depth + 1 {
            return false;
        }

        // Further validation can be added to check for proper scaling and positioning
        // relative to the parent triangle.
    }

    if fractal_tri.depth == 0 {
        return GenesisTriangle::validate(&fractal_tri.triangle);
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::address::TriangleAddress;
    use crate::core::fractal::TriangleState;
    use crate::core::genesis::GenesisTriangle;
    use crate::core::geometry::Point;

    #[test]
    fn test_validate_simple_triangle() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(1), dec!(0));
        let p3 = Point::new(dec!(0), dec!(1));
        let t = Triangle::new(p1, p2, p3);
        assert!(validate_triangle(&t));
    }

    #[test]
    fn test_validate_fractal_triangle() {
        let genesis_tri = GenesisTriangle::new();
        let fractal_tri = FractalTriangle::new(
            genesis_tri,
            TriangleState::Genesis,
            0,
            None,
            TriangleAddress::root(),
        );
        assert!(validate_fractal_triangle(&fractal_tri, None));
    }
}

use crate::core::triangle::Triangle;
use crate::core::errors::TriangleError;

pub fn subdivide_triangle(tri: &Triangle) -> Result<(Triangle, Triangle, Triangle), TriangleError> {
    if !tri.is_valid() {
        return Err(TriangleError::InvalidTriangle("Cannot subdivide a degenerate triangle".to_string()));
    }

    let mid_ab = tri.a.midpoint(&tri.b);
    let mid_bc = tri.b.midpoint(&tri.c);
    let mid_ca = tri.c.midpoint(&tri.a);

    let child1 = Triangle::new(tri.a.clone(), mid_ab.clone(), mid_ca.clone());
    let child2 = Triangle::new(mid_ab.clone(), tri.b.clone(), mid_bc.clone());
    let child3 = Triangle::new(mid_ca.clone(), mid_bc.clone(), tri.c.clone());

    Ok((child1, child2, child3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::genesis::GenesisTriangle;
    use rust_decimal_macros::dec;

    #[test]
    fn test_subdivision() {
        let genesis = GenesisTriangle::new();
        let (c1, c2, c3) = subdivide_triangle(&genesis).unwrap();

        assert!(c1.is_valid());
        assert!(c2.is_valid());
        assert!(c3.is_valid());

        // Test that the area of the children is 1/4 of the parent
        assert!((genesis.area() / dec!(4) - c1.area()).abs() < dec!(1e-9));
    }
}

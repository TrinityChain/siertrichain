
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::core::geometry::Point;
use serde::{Deserialize, Serialize};
use crate::core::hash::H256;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    pub fn area(&self) -> Decimal {
        (self.a.x * (self.b.y - self.c.y) + self.b.x * (self.c.y - self.a.y) + self.c.x * (self.a.y - self.b.y)).abs() / dec!(2)
    }

    pub fn perimeter(&self) -> Decimal {
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }

    pub fn centroid(&self) -> Point {
        Point {
            x: (self.a.x + self.b.x + self.c.x) / dec!(3),
            y: (self.a.y + self.b.y + self.c.y) / dec!(3),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.area() > dec!(0)
    }

    pub fn hash(&self) -> H256 {
        let bytes = bincode::serialize(&self).unwrap();
        blake3::hash(&bytes).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_triangle_new() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(1), dec!(0));
        let p3 = Point::new(dec!(0), dec!(1));
        let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());
        assert_eq!(t.a, p1);
        assert_eq!(t.b, p2);
        assert_eq!(t.c, p3);
    }

    #[test]
    fn test_triangle_area() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(1), dec!(0));
        let p3 = Point::new(dec!(0.5), dec!(0.8660254));
        let t = Triangle::new(p1, p2, p3);
        assert!((t.area() - dec!(0.4330127)).abs() < dec!(1e-6));
    }

    #[test]
    fn test_triangle_perimeter() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(1), dec!(0));
        let p3 = Point::new(dec!(0), dec!(1));
        let t = Triangle::new(p1, p2, p3);
        assert!((t.perimeter() - dec!(3.41421356)).abs() < dec!(1e-6));
    }

    #[test]
    fn test_triangle_centroid() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(3), dec!(0));
        let p3 = Point::new(dec!(0), dec!(3));
        let t = Triangle::new(p1, p2, p3);
        let centroid = t.centroid();
        assert_eq!(centroid.x, dec!(1));
        assert_eq!(centroid.y, dec!(1));
    }

    #[test]
    fn test_triangle_is_valid() {
        let p1 = Point::new(dec!(0), dec!(0));
        let p2 = Point::new(dec!(1), dec!(0));
        let p3 = Point::new(dec!(0), dec!(1));
        let t = Triangle::new(p1, p2, p3);
        assert!(t.is_valid());

        let p4 = Point::new(dec!(0), dec!(0));
        let p5 = Point::new(dec!(1), dec!(1));
        let p6 = Point::new(dec!(2), dec!(2));
        let t2 = Triangle::new(p4, p5, p6);
        assert!(!t2.is_valid());
    }
}

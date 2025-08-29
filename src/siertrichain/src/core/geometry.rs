
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use rust_decimal::MathematicalOps;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: Decimal,
    pub y: Decimal,
}

impl Point {
    pub fn new(x: Decimal, y: Decimal) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> Decimal {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt().unwrap()
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / Decimal::new(2, 0),
            y: (self.y + other.y) / Decimal::new(2, 0),
        }
    }
}

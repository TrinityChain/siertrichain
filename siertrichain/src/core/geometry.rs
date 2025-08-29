
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
}

use crate::core::triangle::Triangle;
use crate::wallet::address::Address;
use crate::core::hash::H256;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents a rental agreement for a triangular region.
pub struct RentalAgreement {
    /// The address of the renter.
    pub renter: Address,
    /// The triangle being rented.
    pub triangle: Triangle,
    /// The rental price.
    pub price: Decimal,
    /// The duration of the rental in blocks.
    pub duration: u64,
    /// The block number when the rental expires.
    pub expiration_block: u64,
}

/// Manages the rental of triangular regions.
pub struct RentalManager {
    /// A map from a triangle identifier to a list of rental agreements.
    rentals: HashMap<H256, Vec<RentalAgreement>>,
}

impl RentalManager {
    pub fn new() -> Self {
        Self { rentals: HashMap::new() }
    }

    /// Creates a new rental agreement for a triangle.
    pub fn rent_triangle(&mut self, renter: Address, triangle: Triangle, price: Decimal, duration: u64, current_block: u64) {
        let triangle_id = triangle.hash();
        let expiration_block = current_block + duration;
        let agreement = RentalAgreement {
            renter,
            triangle,
            price,
            duration,
            expiration_block,
        };
        self.rentals.entry(triangle_id).or_default().push(agreement);
    }

    /// Checks if a triangle is currently rented.
    pub fn is_rented(&self, triangle_id: &H256, current_block: u64) -> bool {
        if let Some(agreements) = self.rentals.get(triangle_id) {
            agreements.iter().any(|agreement| agreement.expiration_block > current_block)
        } else {
            false
        }
    }
}

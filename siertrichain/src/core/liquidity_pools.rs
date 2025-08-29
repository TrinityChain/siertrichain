use crate::core::triangle::Triangle;

use crate::core::hash::H256;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents a liquidity pool for a specific triangular region or depth.
pub struct LiquidityPool {
    /// The triangle associated with this pool. Can be a specific region or a representative for a depth.
    pub triangle: Triangle,
    /// The total amount of the native token in the pool.
    pub token_reserve: Decimal,
    /// The total amount of the triangle asset in the pool.
    pub asset_reserve: Decimal,
}

impl LiquidityPool {
    pub fn new(triangle: Triangle, token_reserve: Decimal, asset_reserve: Decimal) -> Self {
        Self { triangle, token_reserve, asset_reserve }
    }

    /// Calculates the amount of tokens received for a given amount of asset.
    pub fn get_token_amount(&self, asset_amount: Decimal) -> Decimal {
        if self.asset_reserve.is_zero() || self.token_reserve.is_zero() {
            return Decimal::ZERO;
        }
        (self.token_reserve * asset_amount) / self.asset_reserve
    }

    /// Calculates the amount of asset received for a given amount of tokens.
    pub fn get_asset_amount(&self, token_amount: Decimal) -> Decimal {
        if self.token_reserve.is_zero() || self.asset_reserve.is_zero() {
            return Decimal::ZERO;
        }
        (self.asset_reserve * token_amount) / self.token_reserve
    }
}

/// Manages all liquidity pools.
pub struct LiquidityPoolManager {
    pools: HashMap<H256, LiquidityPool>,
}

impl LiquidityPoolManager {
    pub fn new() -> Self {
        Self { pools: HashMap::new() }
    }

    /// Creates a new liquidity pool.
    pub fn create_pool(&mut self, triangle: Triangle, token_reserve: Decimal, asset_reserve: Decimal) {
        let triangle_id = triangle.hash();
        let pool = LiquidityPool::new(triangle, token_reserve, asset_reserve);
        self.pools.insert(triangle_id, pool);
    }

    /// Adds liquidity to an existing pool.
    pub fn add_liquidity(&mut self, triangle_id: &H256, token_amount: Decimal, asset_amount: Decimal) {
        if let Some(pool) = self.pools.get_mut(triangle_id) {
            pool.token_reserve += token_amount;
            pool.asset_reserve += asset_amount;
        }
    }
}

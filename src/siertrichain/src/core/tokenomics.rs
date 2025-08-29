use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::core::triangle::Triangle;

/// The fractal dimension of the Sierpinski triangle (log(3)/log(2)).
pub const SIERPINSKI_FRACTAL_DIMENSION: f64 = 1.584962500721156;

/// Calculates the token reward for a transaction that subdivides a triangle.
/// The reward is proportional to the area of the subdivided triangle.
/// This creates a system where more significant subdivisions yield higher rewards.
pub fn calculate_subdivision_reward(triangle_area: Decimal) -> Decimal {
    // The reward is a fraction of the triangle's area.
    // This parameter can be adjusted as a core consensus rule.
    let reward_factor = dec!(0.01);
    triangle_area * reward_factor
}

/// Calculates the portion of fees to be burned from a transaction.
/// Burning a portion of fees introduces deflationary pressure on the token supply.
pub fn calculate_fee_burn(fee: Decimal) -> Decimal {
    // The burn rate is a fixed percentage of the transaction fee.
    // This can be adjusted as a core consensus rule.
    let burn_rate = dec!(0.1); // 10% of the fee is burned
    fee * burn_rate
}

/// Calculates the mining reward based on the subdivision depth.
/// Rewards decrease geometrically with depth, mirroring the decreasing triangle areas.
pub fn calculate_mining_reward(depth: u32) -> Decimal {
    let base_reward = dec!(100.0);
    let depth_factor = dec!(2.0).powu(depth as u64);
    base_reward / depth_factor
}

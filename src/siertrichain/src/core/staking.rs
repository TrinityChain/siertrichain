use crate::core::triangle::Triangle;
use crate::wallet::address::Address;
use crate::core::hash::H256;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents a stake in a specific triangular region.
pub struct Stake {
    /// The address of the staker.
    pub staker: Address,
    /// The triangle being staked.
    pub triangle: Triangle,
    /// The amount of tokens staked.
    pub amount: Decimal,
}

/// Manages all staking activities within the blockchain.
pub struct StakingManager {
    /// A map from a triangle identifier to a list of stakes in that triangle.
    stakes: HashMap<H256, Vec<Stake>>,
}

impl StakingManager {
    pub fn new() -> Self {
        Self { stakes: HashMap::new() }
    }

    /// Adds a new stake to a specific triangle.
    pub fn add_stake(&mut self, stake: Stake) {
        let triangle_id = stake.triangle.hash();
        self.stakes.entry(triangle_id).or_default().push(stake);
    }

    /// Calculates the staking rewards for a given triangle and subdivision activity.
    /// Rewards are distributed to stakers in proportion to their staked amount.
    pub fn distribute_rewards(&self, triangle_id: &H256, subdivision_reward: Decimal) {
        if let Some(stakes) = self.stakes.get(triangle_id) {
            let total_staked: Decimal = stakes.iter().map(|s| s.amount).sum();

            if total_staked > Decimal::ZERO {
                for stake in stakes {
                    let reward = (stake.amount / total_staked) * subdivision_reward;
                    // In a real implementation, this reward would be transferred to the staker's address.
                    println!("Rewarding staker {:?} with {:?} tokens", stake.staker, reward);
                }
            }
        }
    }
}

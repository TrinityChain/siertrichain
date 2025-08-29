use crate::core::triangle::Triangle;
use crate::wallet::address::Address;
use crate::core::hash::H256;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents an insurance policy for a triangular region.
pub struct InsurancePolicy {
    /// The address of the policyholder.
    pub policyholder: Address,
    /// The triangle being insured.
    pub insured_triangle: Triangle,
    /// The insured value.
    pub insured_value: Decimal,
    /// The premium paid for the insurance.
    pub premium: Decimal,
    /// The block number when the policy expires.
    pub expiration_block: u64,
}

/// Manages the insurance of triangular regions.
pub struct InsuranceManager {
    /// A map from a triangle identifier to a list of insurance policies.
    policies: HashMap<H256, Vec<InsurancePolicy>>,
}

impl InsuranceManager {
    pub fn new() -> Self {
        Self { policies: HashMap::new() }
    }

    /// Creates a new insurance policy for a triangle.
    pub fn create_policy(&mut self, policyholder: Address, insured_triangle: Triangle, insured_value: Decimal, premium: Decimal, expiration_block: u64) {
        let triangle_id = insured_triangle.hash();
        let policy = InsurancePolicy {
            policyholder,
            insured_triangle,
            insured_value,
            premium,
            expiration_block,
        };
        self.policies.entry(triangle_id).or_default().push(policy);
    }

    /// Processes a claim for a subdivision failure or other insured event.
    pub fn process_claim(&mut self, triangle_id: &H256, current_block: u64) {
        // In a real implementation, we would verify the claim and pay it out.
        if let Some(policies) = self.policies.get_mut(triangle_id) {
            policies.retain(|policy| {
                if policy.expiration_block >= current_block {
                    // Payout claim
                    println!("Paying out claim for triangle {:?} to policyholder {:?}", triangle_id, policy.policyholder);
                    false // Remove policy after payout
                } else {
                    true // Keep active policies
                }
            });
        }
    }
}

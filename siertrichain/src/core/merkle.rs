use crate::core::hash::H256;
use crate::core::transaction::Transaction;

pub struct MerkleTree {
    root: H256,
    // In a full implementation, we would store the tree structure
    // to generate proofs.
}

impl MerkleTree {
    pub fn new(transactions: &[Transaction]) -> Self {
        if transactions.is_empty() {
            return Self { root: H256::default() };
        }

        let mut level: Vec<H256> = transactions.iter().map(|tx| tx.hash().clone()).collect();

        while level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;
            while i < level.len() {
                let left = &level[i];
                let right = if i + 1 < level.len() { &level[i+1] } else { left };
                let mut combined_hash_data = [0u8; 64];
                combined_hash_data[..32].copy_from_slice(&left.to_bytes());
                combined_hash_data[32..].copy_from_slice(&right.to_bytes());
                let combined_hash = blake3::hash(&combined_hash_data).into();
                next_level.push(combined_hash);
                i += 2;
            }
            level = next_level;
        }

        Self { root: level[0].clone() }
    }

    pub fn get_root(&self) -> H256 {
        self.root.clone()
    }

    // Methods for generating and verifying proofs would be added here
}

use serde::{Deserialize, Serialize};
use crate::core::transaction::Transaction;
use crate::core::hash::H256;
use chrono::prelude::*;
use crate::core::address::TriangleAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub previous_hash: H256,
    pub merkle_root: H256,
    pub timestamp: i64,
    pub nonce: u64,
    pub difficulty: u64,
    pub height: u64,
    pub geometric_proof: TriangleAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub triangle_transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        previous_hash: H256,
        merkle_root: H256,
        difficulty: u64,
        height: u64,
        triangle_transactions: Vec<Transaction>,
    ) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            header: BlockHeader {
                previous_hash,
                merkle_root,
                timestamp,
                nonce: 0,
                difficulty,
                height,
                geometric_proof: TriangleAddress::root(), // Default value
            },
            triangle_transactions,
        }
    }

    pub fn hash(&self) -> H256 {
        let header_bytes = bincode::serialize(&self.header).unwrap();
        blake3::hash(&header_bytes).into()
    }
}

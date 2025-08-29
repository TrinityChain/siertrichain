use crate::core::block::Block;
use crate::core::blockchain::Blockchain;
use crate::core::triangle::Triangle;
use rust_decimal::Decimal;

pub fn validate_block(block: &Block, blockchain: &Blockchain) -> bool {
    // 1. Check if the previous block exists
    let prev_block = match blockchain.get_block(&block.header.previous_hash) {
        Some(b) => b,
        None => return false, // Previous block not found
    };

    // 2. Validate block timestamp
    if block.header.timestamp <= prev_block.header.timestamp {
        return false;
    }

    // 3. Check proof-of-work
    if !is_valid_proof_of_work(block, blockchain.get_difficulty()) {
        return false;
    }

    // 4. Validate Merkle root

    // 5. Validate all transactions in the block
    for tx in &block.triangle_transactions {
        if !tx.validate() {
            return false;
        }
    }

    true
}

fn is_valid_proof_of_work(block: &Block, difficulty: u64) -> bool {
    let target = u64::MAX / difficulty;
    let hash_value = u64::from_le_bytes(block.hash().to_bytes()[..8].try_into().unwrap());
    hash_value < target
}

pub fn validate_triangle_subdivision(parent: &Triangle, child: &Triangle) -> bool {
    // For now, a simple area check will suffice.
    // A more robust solution would check the coordinates of the child triangle vertices.
    (parent.area() / Decimal::new(4, 0) - child.area()).abs() < Decimal::new(1, 9)
}

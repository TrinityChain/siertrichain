use crate::core::blockchain::Blockchain;

const TARGET_BLOCK_TIME: u64 = 60; // 1 minute
const DIFFICULTY_ADJUSTMENT_INTERVAL: u64 = 10; // 10 blocks

pub fn adjust_difficulty(blockchain: &Blockchain) -> u64 {
    let latest_block = blockchain.latest_block();
    if latest_block.header.height % DIFFICULTY_ADJUSTMENT_INTERVAL != 0 {
        return blockchain.get_difficulty();
    }

    let previous_adjustment_block = blockchain.get_block_by_height(latest_block.header.height - DIFFICULTY_ADJUSTMENT_INTERVAL).unwrap();

    let time_taken = latest_block.header.timestamp - previous_adjustment_block.header.timestamp;
    let expected_time = (DIFFICULTY_ADJUSTMENT_INTERVAL * TARGET_BLOCK_TIME) as i64;

    let new_difficulty = if time_taken < expected_time / 2 {
        blockchain.get_difficulty() * 2
    } else if time_taken > expected_time * 2 {
        blockchain.get_difficulty() / 2
    } else {
        blockchain.get_difficulty()
    };

    // Ensure difficulty doesn't go to zero or become excessively large.
    new_difficulty.max(1).min(u64::MAX / 1000)
}
use crate::core::block::{Block};
use crate::core::hash::H256;
use crate::core::genesis::GenesisTriangle;
use crate::core::merkle::MerkleTree;
use crate::core::transaction::Transaction;
use std::collections::HashMap;
use crate::core::fractal::FractalTriangle;

pub struct Blockchain {
    blocks: Vec<Block>,
    block_map: HashMap<H256, usize>,
    difficulty: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Self {
            blocks: Vec::new(),
            block_map: HashMap::new(),
            difficulty: 1_000_000, // Initial difficulty
        };
        let genesis_block = blockchain.create_genesis_block();
        blockchain.add_block(genesis_block).unwrap();
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        let genesis_triangle = GenesisTriangle::new();
        // In a real implementation, the genesis transaction would be more meaningful.
        let genesis_tx = Transaction::new_genesis(genesis_triangle);
        let transactions = vec![genesis_tx];
        let merkle_root = MerkleTree::new(&transactions).get_root();
        let mut genesis_block = Block::new(H256::default(), merkle_root, self.difficulty, 0, transactions);
        self.find_nonce(&mut genesis_block);
        genesis_block
    }

    pub fn add_block(&mut self, mut block: Block) -> Result<(), &'static str> {
        if !self.is_valid_block(&block) {
            return Err("Invalid block");
        }

        self.find_nonce(&mut block);
        let block_hash = block.hash();
        self.block_map.insert(block_hash, self.blocks.len());
        self.blocks.push(block);

        // Adjust difficulty
        if self.blocks.len() % 10 == 0 { // Adjust every 10 blocks
            self.adjust_difficulty();
        }

        Ok(())
    }

    fn is_valid_block(&self, block: &Block) -> bool {
        let last_block = self.latest_block();
        if block.header.previous_hash != last_block.hash() {
            return false; // Previous hash doesn't match
        }

        // Add more validation logic here (e.g. timestamp, difficulty)

        true
    }

    fn find_nonce(&self, block: &mut Block) {
        while !self.is_valid_proof_of_work(block) {
            block.header.nonce += 1;
        }
    }

    fn is_valid_proof_of_work(&self, block: &Block) -> bool {
        let target = u64::MAX / self.difficulty;
        let hash_value = u64::from_le_bytes(block.hash().to_bytes()[..8].try_into().unwrap());
        hash_value < target
    }

    pub fn get_block(&self, hash: &H256) -> Option<&Block> {
        self.block_map.get(hash).map(|&index| &self.blocks[index])
    }

    pub fn get_difficulty(&self) -> u64 {
        self.difficulty
    }

    fn adjust_difficulty(&mut self) {
        // Simplified difficulty adjustment algorithm
        let last_adjustment_block = &self.blocks[self.blocks.len() - 10];
        let time_taken = self.latest_block().header.timestamp - last_adjustment_block.header.timestamp;
        let expected_time = 10 * 60; // 10 minutes

        if time_taken < expected_time / 2 {
            self.difficulty *= 2;
        } else if time_taken > expected_time * 2 {
            self.difficulty /= 2;
        }

        if self.difficulty == 0 { // Ensure difficulty is never zero
            self.difficulty = 1;
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn get_block_by_height(&self, height: u64) -> Option<&Block> {
        self.blocks.get(height as usize)
    }

    pub fn get_active_triangles(&self) -> Vec<FractalTriangle> {
        let mut active_triangles = Vec::new();
        for block in &self.blocks {
            for tx in &block.triangle_transactions {
                active_triangles.extend(tx.get_fractal_triangles());
            }
        }
        active_triangles
    }
}

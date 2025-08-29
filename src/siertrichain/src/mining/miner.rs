use crate::core::block::{Block};
use crate::core::blockchain::Blockchain;
use crate::core::merkle::MerkleTree;
use crate::core::transaction::Transaction;
use crate::mining::config::MiningConfig;
use crate::core::hash::H256;


pub struct Miner {
    config: MiningConfig,
    blockchain: Blockchain,
}

impl Miner {
    pub fn new(config: MiningConfig, blockchain: Blockchain) -> Self {
        Self { config, blockchain }
    }

    pub fn mine(&mut self) -> Block {
        let mut candidate_block = self.generate_candidate_block();
        self.find_geometric_proof(&mut candidate_block);
        candidate_block
    }

    fn generate_candidate_block(&self) -> Block {
        let triangle_transactions: Vec<Transaction> = Vec::new(); 
        let last_block = self.blockchain.latest_block();
        let merkle_root = MerkleTree::new(&triangle_transactions).get_root();
        let height = last_block.header.height + 1;
        Block::new(
            last_block.hash(),
            merkle_root,
            self.blockchain.get_difficulty(),
            height,
            triangle_transactions,
        )
    }

    fn find_geometric_proof(&self, block: &mut Block) {
        let difficulty = self.config.difficulty_target;
        let mut nonce: u64 = 0;

        loop {
            let mut active_triangles = self.blockchain.get_active_triangles();

            for fractal_triangle in &mut active_triangles {
                if let Ok(children) = fractal_triangle.subdivide() {
                    for child in children {
                        let mut block_header = block.header.clone();
                        block_header.nonce = nonce;
                        block_header.geometric_proof = child.address.clone();

                        let header_bytes = bincode::serialize(&block_header).unwrap();
                        let hash = blake3::hash(&header_bytes).into();

                        if self.validate_geometric_work(&hash, difficulty) {
                            block.header = block_header;
                            return;
                        }
                        nonce += 1;
                    }
                }
            }
        }
    }


    fn validate_geometric_work(&self, hash: &H256, difficulty: u64) -> bool {
        let hash_value = u64::from_le_bytes(hash.to_bytes()[..8].try_into().unwrap());
        hash_value < u64::MAX / difficulty
    }
}

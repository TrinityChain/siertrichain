use crate::core::block::Block;
use crate::core::hash::H256;
use crate::core::transaction::Transaction;
use std::collections::HashMap;

pub trait BlockStorage {
    fn get_block(&self, hash: &H256) -> Option<Block>;
    fn put_block(&mut self, block: &Block);
}

pub trait TransactionStorage {
    fn get_transaction(&self, hash: &H256) -> Option<Transaction>;
    fn put_transaction(&mut self, tx: &Transaction);
}

pub struct InMemoryStorage {
    blocks: HashMap<H256, Block>,
    transactions: HashMap<H256, Transaction>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            transactions: HashMap::new(),
        }
    }
}

impl BlockStorage for InMemoryStorage {
    fn get_block(&self, hash: &H256) -> Option<Block> {
        self.blocks.get(hash).cloned()
    }

    fn put_block(&mut self, block: &Block) {
        self.blocks.insert(block.hash(), block.clone());
    }
}

impl TransactionStorage for InMemoryStorage {
    fn get_transaction(&self, hash: &H256) -> Option<Transaction> {
        self.transactions.get(hash).cloned()
    }

    fn put_transaction(&mut self, tx: &Transaction) {
        self.transactions.insert(tx.hash().clone(), tx.clone());
    }
}

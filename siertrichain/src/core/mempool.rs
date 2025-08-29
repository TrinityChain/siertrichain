use crate::core::transaction::Transaction;
use std::collections::HashMap;

pub struct Mempool {
    transactions: HashMap<[u8; 32], Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { transactions: HashMap::new() }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.insert(tx.hash().to_bytes(), tx);
    }

    pub fn get_transaction(&self, hash: &[u8; 32]) -> Option<&Transaction> {
        self.transactions.get(hash)
    }

    pub fn get_pending_transactions(&self) -> Vec<Transaction> {
        self.transactions.values().cloned().collect()
    }
}

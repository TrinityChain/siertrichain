use crate::core::transaction::{Transaction, TriangleOperation};
use ed25519_dalek::{Keypair, PublicKey};


pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub fn new_random() -> Self {
        let mut csprng = rand::thread_rng();
        let keypair = Keypair::generate(&mut csprng);
        Self { keypair }
    }

    pub fn address(&self) -> PublicKey {
        self.keypair.public
    }

    pub fn create_transaction(&self, from: crate::core::address::TriangleAddress, to: PublicKey) -> Transaction {
        let operation = TriangleOperation::Transfer {
            from,
            to,
        };
        Transaction::new(operation, &self.keypair)
    }
}

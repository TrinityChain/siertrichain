use ed25519_dalek::{Keypair, SignatureError};


pub fn from_bytes(bytes: &[u8]) -> Result<Keypair, SignatureError> {
    Keypair::from_bytes(bytes)
}

pub fn to_bytes(keypair: &Keypair) -> [u8; 64] {
    keypair.to_bytes()
}

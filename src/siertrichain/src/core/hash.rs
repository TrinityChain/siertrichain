use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut data = [0; 32];
        data.copy_from_slice(slice);
        Self(data)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl From<[u8; 32]> for H256 {
    fn from(hash: [u8; 32]) -> Self {
        Self(hash)
    }
}

impl From<blake3::Hash> for H256 {
    fn from(hash: blake3::Hash) -> Self {
        Self(*hash.as_bytes())
    }
}

impl fmt::Debug for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

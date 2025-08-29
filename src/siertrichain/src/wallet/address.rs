use ed25519_dalek::PublicKey;
use bs58;
use thiserror::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Address {
    // In a real implementation, this would contain more structured data.
    data: [u8; 33], 
}

#[derive(Error, Debug, PartialEq)]
pub enum AddressError {
    #[error("Invalid base58 string: {0}")]
    InvalidBase58(#[from] bs58::decode::Error),
    #[error("Invalid address length: {0}")]
    InvalidLength(usize),
}

impl Address {
    pub fn from_pubkey(pubkey: &PublicKey) -> Self {
        // This is a simplified address generation scheme.
        let mut data = [0u8; 33];
        data[0] = 0; // Version byte
        data[1..].copy_from_slice(&pubkey.to_bytes());
        Self { data }
    }

    pub fn to_base58(&self) -> String {
        bs58::encode(&self.data).into_string()
    }

    pub fn from_base58(s: &str) -> Result<Self, AddressError> {
        let bytes = bs58::decode(s).into_vec()?;
        if bytes.len() != 33 {
            return Err(AddressError::InvalidLength(bytes.len()));
        }
        let mut data = [0u8; 33];
        data.copy_from_slice(&bytes);
        Ok(Self { data })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.data)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: &[u8] = serde::Deserialize::deserialize(deserializer)?;
        let data: [u8; 33] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::invalid_length(bytes.len(), &"an array of 33 bytes"))?;
        Ok(Address { data })
    }
}

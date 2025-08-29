
use serde::{Deserialize, Serialize};
use crate::core::address::TriangleAddress;
use crate::core::hash::H256;

/// Represents a Non-Fungible Token (NFT) tied to a specific triangular coordinate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriangleNFT {
    /// A unique identifier for the token.
    pub token_id: H256,
    /// The geometric address of the triangle this NFT represents.
    pub triangle_address: TriangleAddress,
    /// The public key of the owner of this NFT.
    pub owner: String,
    /// A URI pointing to off-chain metadata for this NFT (e.g., an image or JSON file).
    pub metadata_uri: String,
}

impl TriangleNFT {
    /// Creates a new Triangle NFT.
    ///
    /// # Arguments
    ///
    /// * `triangle_address` - The `TriangleAddress` of the fractal triangle.
    /// * `owner` - The public key of the owner.
    /// * `metadata_uri` - A URI for the NFT's metadata.
    ///
    /// # Returns
    ///
    /// A new `TriangleNFT` instance.
    pub fn new(triangle_address: TriangleAddress, owner: String, metadata_uri: String) -> Self {
        let mut nft = Self {
            token_id: H256::default(), // Placeholder, will be calculated from content.
            triangle_address,
            owner,
            metadata_uri,
        };
        nft.token_id = nft.calculate_hash();
        nft
    }

    /// Calculates the hash of the NFT, which serves as its unique token_id.
    fn calculate_hash(&self) -> H256 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.triangle_address.to_string().as_bytes());
        bytes.extend_from_slice(self.owner.as_bytes());
        bytes.extend_from_slice(self.metadata_uri.as_bytes());
        let hash = blake3::hash(&bytes);
        H256::from(hash)
    }
}

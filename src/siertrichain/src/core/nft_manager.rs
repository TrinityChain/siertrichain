use std::collections::HashMap;
use crate::core::address::TriangleAddress;
use crate::core::nft::TriangleNFT;
use crate::core::errors::BlockchainError;
use crate::core::hash::H256;

/// Manages the lifecycle of Triangle NFTs on the blockchain.
pub struct NFTManager {
    /// A mapping from a `TriangleAddress` to the `token_id` of the NFT that occupies it.
    triangle_to_nft: HashMap<TriangleAddress, H256>,
    /// A mapping from a `token_id` to the `TriangleNFT` data.
    nfts: HashMap<H256, TriangleNFT>,
}

impl NFTManager {
    /// Creates a new `NFTManager`.
    pub fn new() -> Self {
        Self {
            triangle_to_nft: HashMap::new(),
            nfts: HashMap::new(),
        }
    }

    /// Mints a new NFT and associates it with a triangle address.
    ///
    /// # Arguments
    ///
    /// * `triangle_address` - The address of the triangle for the new NFT.
    /// * `owner` - The public key of the new owner.
    /// * `metadata_uri` - The URI for the NFT's metadata.
    ///
    /// # Returns
    ///
    /// The newly created `TriangleNFT`.
    pub fn mint_nft(
        &mut self,
        triangle_address: TriangleAddress,
        owner: String,
        metadata_uri: String,
    ) -> Result<TriangleNFT, BlockchainError> {
        if self.triangle_to_nft.contains_key(&triangle_address) {
            return Err(BlockchainError::TriangleOccupied);
        }

        let nft = TriangleNFT::new(triangle_address.clone(), owner, metadata_uri);
        let token_id = nft.token_id.clone();

        self.nfts.insert(token_id.clone(), nft.clone());
        self.triangle_to_nft.insert(triangle_address, token_id);

        Ok(nft)
    }

    /// Transfers an NFT to a new owner.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The ID of the NFT to transfer.
    /// * `new_owner` - The public key of the new owner.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the transfer was successful.
    pub fn transfer_nft(&mut self, token_id: &H256, new_owner: String) -> Result<(), BlockchainError> {
        let nft = self.nfts.get_mut(token_id).ok_or(BlockchainError::NFTNotFound)?;
        nft.owner = new_owner;
        Ok(())
    }

    /// Gets the NFT associated with a given triangle address.
    ///
    /// # Arguments
    ///
    /// * `triangle_address` - The address to query.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `TriangleNFT` if it exists.
    pub fn get_nft_by_triangle(&self, triangle_address: &TriangleAddress) -> Option<&TriangleNFT> {
        let token_id = self.triangle_to_nft.get(triangle_address)?;
        self.nfts.get(token_id)
    }
}

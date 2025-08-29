use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriangleError {
    #[error("Invalid address format")]
    InvalidAddressFormat,
    #[error("Invalid triangle: {0}")]
    InvalidTriangle(String),
}

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Triangle is already occupied by an NFT")]
    TriangleOccupied,
    #[error("NFT not found")]
    NFTNotFound,
}

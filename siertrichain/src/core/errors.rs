
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriangleError {
    #[error("Invalid address format")]
    InvalidAddressFormat,
}

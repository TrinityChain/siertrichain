use serde::{Deserialize, Serialize};
use crate::core::block::Block;
use crate::core::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    GetPeers,
    Peers(Vec<String>),
    GetBlocks(Vec<[u8; 32]>),
    Blocks(Vec<Block>),
    NewTransaction(Transaction),
}

// The wire protocol will be a simple length-prefixed, bincode-serialized
// stream of messages.

pub fn serialize_message(msg: &Message) -> Vec<u8> {
    bincode::serialize(msg).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> Result<Message, bincode::Error> {
    bincode::deserialize(data)
}

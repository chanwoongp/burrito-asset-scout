use serde::Deserialize;
use crate::types::IBlock;

// Struct matching the JSON-RPC wire format from Ethereum nodes.
#[derive(Debug, Deserialize)]
pub struct GetBlockNumberResponse {
    pub number: Option<String>,
    pub hash: Option<String>,
}

pub struct BlockMetadata {
    pub hash: String,
    pub number: u64,
}

pub struct Transaction {
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub value: u64,
}

pub struct Block;

impl IBlock for Block {
    type BlockMetadata = BlockMetadata;
    type Transaction = Transaction;
}
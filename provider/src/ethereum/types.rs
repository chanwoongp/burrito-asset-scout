use serde::Deserialize;
use crate::types::IBlockData;

#[derive(Debug, Deserialize)]
pub struct GetBlockNumberResponse {
    pub number: Option<String>,
    pub hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockHeader {
    pub hash: String,
    pub number: u64,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub value: u64,
}

pub struct BlockData;

impl IBlockData for BlockData {
    type BlockHeader = BlockHeader;
    type Transaction = Transaction;
}
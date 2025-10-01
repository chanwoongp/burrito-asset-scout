use serde::Deserialize;
use crate::types::IBlockData;

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashResponse {
    pub context: Context,
    pub value: GetLatestBlockHashValue,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    // #[serde(rename = "apiVersion")]
    // pub api_version: Option<String>,
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashValue {
    pub blockhash: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockHeader {
    pub block_height: u64,
    pub block_time: u64,
    pub blockhash: String,
    pub parent_lot: u64,
    pub previous_blockhash: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub block_time: u64,
    pub slot: u64,
}

pub struct BlockData;

impl IBlockData for BlockData {
    type BlockHeader = BlockHeader;
    type Transaction = Transaction;
}

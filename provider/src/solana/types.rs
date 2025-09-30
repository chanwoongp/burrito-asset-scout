use serde::Deserialize;
use crate::types::IBlock;

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockhashResponse {
    pub context: Context,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    //#[serde(rename = "apiVersion")]
    //pub api_version: Option<String>,
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub blockhash: String,
    //#[serde(rename = "lastValidBlockHeight")]
    //pub last_valid_block_height: u64,
}

pub struct BlockMetadata {
    pub block_height: u64,
    pub block_time: u64,
    pub blockhash: String,
    pub parent_lot: u64,
    pub previous_blockhash: String,
}

pub struct Transaction {
    pub block_time: u64,
    pub slot: u64,
}

pub struct Block;

impl IBlock for Block {
    type BlockMetadata = BlockMetadata;
    type Transaction = Transaction;
}

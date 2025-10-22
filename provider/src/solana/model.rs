use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

// GetLatestBlockHash

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashResponse {
    pub context: Context,
    pub value: GetLatestBlockHashValue,
}

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashValue {
    pub blockhash: String,
}

// GetBlockResponse (relaxed to accept any transaction shape)

#[derive(Serialize, Deserialize)]
pub(crate) struct GetBlockResponse {
    #[serde(rename = "blockHeight")]
    pub block_height: i64,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
    pub blockhash: String,
    #[serde(rename = "parentSlot")]
    pub parent_slot: i64,
    #[serde(rename = "previousBlockhash")]
    pub previous_blockhash: String,
    pub transactions: Vec<Value>,
}

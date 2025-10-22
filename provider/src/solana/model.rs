use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

#[derive(Serialize, Deserialize)]
struct Status {
    #[serde(rename = "Ok")]
    pub ok: Option<bool>,
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

// GetBlockResponse

#[derive(Serialize, Deserialize)]
struct GetBlockResponseTransactionAccountKey {
    pub pubkey: String,
    pub signer: bool,
    pub source: String,
    pub writable: bool,
}

#[derive(Serialize, Deserialize)]
struct GetBlockResponseTransactionItem {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<GetBlockResponseTransactionAccountKey>,
    pub signatures: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GetBlockResponseTransactionMeta {
    pub err: Option<String>,
    pub fee: i64,
    #[serde(rename = "postBalances")]
    pub post_balances: Vec<i64>,
    #[serde(rename = "postTokenBalances")]
    pub post_token_balances: Vec<u64>,
    #[serde(rename = "preBalances")]
    pub pre_balances: Vec<i64>,
    #[serde(rename = "preTokenBalances")]
    pub pre_token_balances: Vec<u64>,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
struct GetBlockResponseTransaction {
    pub meta: GetBlockResponseTransactionMeta,
    pub transaction: GetBlockResponseTransactionItem,
    pub version: String,
}

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
    pub transactions: Vec<GetBlockResponseTransaction>,
}

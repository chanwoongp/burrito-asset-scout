use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashResponse {
    pub context: Context,
    pub value: GetLatestBlockHashValue,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetLatestBlockHashValue {
    pub blockhash: String,
}

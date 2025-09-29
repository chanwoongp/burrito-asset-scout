use serde::Deserialize;

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

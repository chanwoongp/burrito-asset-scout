use serde::Deserialize;

// Struct matching the JSON-RPC wire format from Ethereum nodes.
#[derive(Debug, Deserialize)]
pub struct GetBlockNumberResponse {
    pub number: Option<String>,
    pub hash: Option<String>,
}

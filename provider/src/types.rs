use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
}

impl Config {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    #[allow(unused)]
    pub jsonrpc: Option<String>,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
    #[allow(unused)]
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

// Struct for internal use after parsing values from the wire format.
#[derive(Debug)]
pub struct BlockHeader {
    pub number: Option<u64>,
    pub hash: Option<String>,
}

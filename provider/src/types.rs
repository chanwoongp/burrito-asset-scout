use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct BlockMinimal {
    pub number: Option<String>,
    pub hash: Option<String>,
}

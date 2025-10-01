use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug)]
pub struct JsonRpc {
    client: reqwest::Client,
    endpoint: String,
}

impl JsonRpc {
    pub fn new(endpoint: String) -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self { client, endpoint })
    }

    pub async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 0
        });

        let resp = self.client.post(&self.endpoint).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(anyhow!("HTTP error: {}", resp.status()));
        }

        let rpc: JsonRpcResponse<T> = resp.json().await?;
        if let Some(err) = rpc.error {
            return Err(anyhow!("JSON-RPC error {}: {}", err.code, err.message));
        }

        rpc.result
            .ok_or_else(|| anyhow!("No result in JSON-RPC response"))
    }
}

#[macro_export]
macro_rules! make_params {
    ($($param:tt),* $(,)?) => {
        serde_json::json!([$($param),*])
    };
}

pub use make_params;

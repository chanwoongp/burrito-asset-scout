use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json;

use crate::types::{BlockMinimal, JsonRpcResponse};
use crate::{BlockHeader, Rpc};

#[derive(Debug)]
pub struct Ethereum;

fn parse_hex_u64(s: &str) -> Result<u64> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|e| anyhow!("failed to parse hex u64: {}", e))
}


impl Rpc for Ethereum {
    async fn fetch_latest_block(&self, client: &Client, endpoint: &str) -> Result<Option<BlockHeader>> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": ["latest", false],
            "id": 0
        });

        let resp = client.post(endpoint).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(anyhow!("HTTP error: {}", resp.status()));
        }

        let rpc: JsonRpcResponse<BlockMinimal> = resp.json().await?;

        if let Some(err) = rpc.error {
            return Err(anyhow!("JSON-RPC error {}: {}", err.code, err.message));
        }

        match rpc.result {
            None => Ok(None),
            Some(b) => {
                let number = match b.number.as_deref() {
                    Some(s) => Some(parse_hex_u64(s)?),
                    None => None,
                };
                let header = BlockHeader { number, hash: b.hash };
                Ok(Some(header))
            }
        }
    }
}


mod types;

pub use types::{BlockMetadata, Transaction, Block};

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json;
use crate::ethereum::types::GetBlockNumberResponse;
use crate::{Rpc};
use crate::types::{BlockHeader, JsonRpcResponse, Config};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Ethereum {
    client: Client,
    config: Config,
}

impl Ethereum {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder().build()?;
        Ok(Self { client, config })
    }
}

fn parse_hex_u64(s: &str) -> Result<u64> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|e| anyhow!("failed to parse hex u64: {}", e))
}


#[async_trait]
impl Rpc for Ethereum {
    async fn fetch_block_info(&self, _block_number: u64) -> Result<Option<crate::types::AnyBlock>> {
        // FIXME
        let block = crate::types::AnyBlock::Ethereum(crate::types::Block::<Block> {
            metadata: BlockMetadata {
                hash: "0x111".to_string(),
                number: 111
            },
            transactions: vec![],
        });
        Ok(Some(block))
    }

    async fn fetch_latest_block_info(&self) -> Result<Option<BlockHeader>> {
        let body = utils::rpc_request!("eth_getBlockByNumber", "latest", false);
        let resp = self.client.post(&self.config.endpoint).json(&body).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("HTTP error: {}", resp.status()));
        }

        let rpc: JsonRpcResponse<GetBlockNumberResponse> = resp.json().await?;
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


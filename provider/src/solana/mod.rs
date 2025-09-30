mod types;

pub use types::{BlockMetadata, Transaction, Block};

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json;
use crate::solana::types::GetLatestBlockhashResponse;
use crate::Rpc;
use crate::types::{BlockHeader, JsonRpcResponse, Config};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Solana {
    client: Client,
    config: Config,
}

impl Solana {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder().build()?;
        Ok(Self { client, config })
    }
}

#[async_trait]
impl Rpc for Solana {
    async fn fetch_block_info(&self, _block_number: u64) -> Result<Option<crate::types::AnyBlock>> {
        let block = crate::types::AnyBlock::Solana(crate::types::Block::<Block> {
            metadata: BlockMetadata {
                blockhash: "0x222".to_string(),
                parent_lot: 0,
                block_height: 222,
                block_time: 0,
                previous_blockhash: "".to_string(),
            },
            transactions: vec![],
        });
        Ok(Some(block))
    }

    async fn fetch_latest_block_info(&self) -> Result<Option<BlockHeader>> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "getLatestBlockhash",
            "params": [{
                "commitment":"finalized"
            }],
            "id": 0
        });
        let resp = self.client.post(&self.config.endpoint).json(&body).send().await?;

        if !resp.status().is_success() {
            return Err(anyhow!("HTTP error: {}", resp.status()));
        }

        let rpc: JsonRpcResponse<GetLatestBlockhashResponse> = resp.json().await?;

        if let Some(err) = rpc.error {
            return Err(anyhow!("JSON-RPC error {}: {}", err.code, err.message));
        }

        match rpc.result {
            None => Ok(None),
            Some(b) => {
                let number = Some(b.context.slot);
                let hash = Some(b.value.blockhash);
                let header = BlockHeader { number, hash };
                Ok(Some(header))
            }
        }
    }
}


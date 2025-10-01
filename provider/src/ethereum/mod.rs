mod types;
mod models;

pub use types::{BlockData, BlockHeader, Transaction};

use crate::Rpc;
use crate::types::{LatestBlockInfo, Config};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use utils::json;
use utils::json::JsonRpc;
use crate::ethereum::models::{GetBlockByNumberResponse, GetTransactionByHashResponse};

#[derive(Debug)]
pub struct Ethereum {
    rpc: JsonRpc,
}

impl Ethereum {
    pub fn new(config: Config) -> Result<Self> {
        let rpc = JsonRpc::new(config.endpoint)?;
        Ok(Self { rpc })
    }

    async fn get_block_by_number(&self, n: u64, include_transaction: bool) -> Result<GetBlockByNumberResponse>{
        let block_param = if n == 0 {
            "latest".to_string()
        } else {
            format!("0x{:x}", n)
        };
        let result: GetBlockByNumberResponse = self
            .rpc
            .request("eth_getBlockByNumber", json::make_params!(block_param, include_transaction))
            .await?;
        Ok(result)
    }

    async fn _get_transaction_by_hash(&self, transaction_hash: String) -> Result<GetTransactionByHashResponse>{
        let result: GetTransactionByHashResponse = self
            .rpc
            .request("eth_getTransactionByHash", json::make_params!(transaction_hash))
            .await?;
        Ok(result)
    }
}

fn parse_hex_u64(s: &str) -> Result<u64> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|e| anyhow!("failed to parse hex u64: {}", e))
}

#[async_trait]
impl Rpc for Ethereum {
    async fn fetch_latest_block_info(&self) -> Result<Option<LatestBlockInfo>> {
        let result = self.get_block_by_number(0, false).await?;
        let header = LatestBlockInfo {
            number: parse_hex_u64(&result.number)?,
            hash: result.hash,
        };
        Ok(Some(header))
    }

    async fn fetch_block_data(&self, block_number: u64) -> Result<Option<crate::types::AnyBlockData>> {
        let result = self.get_block_by_number(block_number, true).await?;
        let block = crate::types::AnyBlockData::Ethereum(crate::types::BlockData::<BlockData> {
            block_header: BlockHeader {
                hash: result.hash,
                parent_hash: result.parent_hash,
                number: result.number.parse()?,
            },
            transactions: vec![],
        });
        Ok(Some(block))
    }
}

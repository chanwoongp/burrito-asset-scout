mod types;

pub use types::{BlockData, BlockHeader, Transaction};

use crate::Rpc;
use crate::ethereum::types::GetBlockNumberResponse;
use crate::types::{LatestBlockInfo, Config};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use utils::json;
use utils::json::JsonRpc;

#[derive(Debug)]
pub struct Ethereum {
    rpc: JsonRpc,
}

impl Ethereum {
    pub fn new(config: Config) -> Result<Self> {
        let rpc = JsonRpc::new(config.endpoint)?;
        Ok(Self { rpc })
    }
}

fn parse_hex_u64(s: &str) -> Result<u64> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|e| anyhow!("failed to parse hex u64: {}", e))
}

#[async_trait]
impl Rpc for Ethereum {
    async fn fetch_latest_block_info(&self) -> Result<Option<LatestBlockInfo>> {
        let result: GetBlockNumberResponse = self
            .rpc
            .request("eth_getBlockByNumber", json::make_params!("latest", false))
            .await?;

        let number = match result.number.as_deref() {
            Some(s) => Some(parse_hex_u64(s)?),
            None => None,
        };
        let header = LatestBlockInfo {
            number,
            hash: result.hash,
        };
        Ok(Some(header))
    }

    async fn fetch_block_data(&self, _block_number: u64) -> Result<Option<crate::types::AnyBlockData>> {
        // FIXME
        let block = crate::types::AnyBlockData::Ethereum(crate::types::BlockData::<BlockData> {
            block_header: BlockHeader {
                hash: "0x111".to_string(),
                number: 111,
            },
            transactions: vec![],
        });
        Ok(Some(block))
    }
}

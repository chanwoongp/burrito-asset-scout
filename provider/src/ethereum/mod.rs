mod types;

pub use types::{BlockMetadata, Transaction, Block};

use anyhow::{anyhow, Result};
use crate::ethereum::types::GetBlockNumberResponse;
use crate::Rpc;
use crate::types::{BlockHeader, Config};
use async_trait::async_trait;
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
        let result: GetBlockNumberResponse = self.rpc.request(
            "eth_getBlockByNumber",
            utils::make_params!(["latest", false])
        ).await?;

        let number = match result.number.as_deref() {
            Some(s) => Some(parse_hex_u64(s)?),
            None => None,
        };
        let header = BlockHeader { number, hash: result.hash };
        Ok(Some(header))
    }
}


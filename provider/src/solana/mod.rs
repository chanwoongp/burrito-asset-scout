mod types;

pub use types::{BlockMetadata, Transaction, Block};

use anyhow::Result;
use serde_json;
use crate::solana::types::GetLatestBlockhashResponse;
use crate::Rpc;
use crate::types::{BlockHeader, Config};
use async_trait::async_trait;
use utils::json::JsonRpc;

#[derive(Debug)]
pub struct Solana {
    rpc: JsonRpc,
}

impl Solana {
    pub fn new(config: Config) -> Result<Self> {
        let rpc = JsonRpc::new(config.endpoint)?;
        Ok(Self { rpc })
    }
}

#[async_trait]
impl Rpc for Solana {
    async fn fetch_block_info(&self, _block_number: u64) -> Result<Option<crate::types::AnyBlock>> {
        // FIXME
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
        let result: GetLatestBlockhashResponse = self.rpc.request(
            "getLatestBlockhash",
            serde_json::json!({"commitment": "finalized"})
        ).await?;

        let number = Some(result.context.slot);
        let hash = Some(result.value.blockhash);
        let header = BlockHeader { number, hash };
        Ok(Some(header))
    }
}


mod types;

pub use types::{BlockData, BlockHeader, Transaction};

use crate::Rpc;
use crate::solana::types::GetLatestBlockHashResponse;
use crate::types::{LatestBlockInfo, Config};
use anyhow::Result;
use async_trait::async_trait;
use serde_json;
use utils::json;
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
    async fn fetch_latest_block_info(&self) -> Result<Option<LatestBlockInfo>> {
        let result: GetLatestBlockHashResponse = self
            .rpc
            .request(
                "getLatestBlockhash",
                json::make_params!({"commitment": "finalized"}),
            )
            .await?;

        let number = Some(result.context.slot);
        let hash = Some(result.value.blockhash);
        let header = LatestBlockInfo { number, hash };
        Ok(Some(header))
    }

    async fn fetch_block_data(&self, _block_number: u64) -> Result<Option<crate::types::AnyBlockData>> {
        // FIXME
        let block = crate::types::AnyBlockData::Solana(crate::types::BlockData::<BlockData> {
            block_header: BlockHeader {
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
}

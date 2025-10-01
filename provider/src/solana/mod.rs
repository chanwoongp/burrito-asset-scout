mod model;
mod types;

pub use types::{BlockData, BlockHeader, Transaction};

use crate::solana::model::GetLatestBlockHashResponse;
use crate::types::{Config, LatestBlockInfo};
use crate::Rpc;
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

    async fn get_latest_blockhash(&self) -> Result<GetLatestBlockHashResponse> {
        let result: GetLatestBlockHashResponse = self
            .rpc
            .request(
                "eth_getBlockByNumber",
                json::make_params!({"commitment": "finalized"}),
            )
            .await?;
        Ok(result)
    }
}

#[async_trait]
impl Rpc for Solana {
    async fn fetch_latest_block_info(&self) -> Result<Option<LatestBlockInfo>> {
        let result = self.get_latest_blockhash().await?;
        let header = LatestBlockInfo {
            number: result.context.slot,
            hash: result.value.blockhash,
        };
        Ok(Some(header))
    }

    async fn fetch_block_data(
        &self,
        _block_number: u64,
    ) -> Result<Option<crate::types::AnyBlockData>> {
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

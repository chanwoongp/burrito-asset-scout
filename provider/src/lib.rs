pub mod ethereum;
pub mod solana;
pub mod types;
pub mod config;

pub use types::{Chain, Config};
pub use config::Providers;

use anyhow::Result;
use crate::types::LatestBlockInfo;
use async_trait::async_trait;

pub fn new_provider(chain: Chain, providers: &Providers) -> Result<Box<dyn Rpc + Send + Sync>> {
    match chain {
        Chain::Ethereum => {
            let endpoint = providers.ethereum.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let eth = ethereum::Ethereum::new(config)?;
            Ok(Box::new(eth))
        }
        Chain::Solana => {
            let endpoint = providers.solana.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let sol = solana::Solana::new(config)?;
            Ok(Box::new(sol))
        }
    }
}

#[async_trait]
pub trait Rpc: Send + Sync {
    async fn fetch_latest_block_info(&self) -> Result<Option<LatestBlockInfo>>;
    async fn fetch_block_data(&self, block_number: u64) -> Result<Option<types::AnyBlockData>>;
}
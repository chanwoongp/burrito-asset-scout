pub mod ethereum;
pub mod solana;
pub mod types;
pub mod config;

pub use types::Config;
pub use config::Providers;

use anyhow::{Result, anyhow};
use crate::types::BlockHeader;
use async_trait::async_trait;

pub fn new_client(chain: &str, providers: &Providers) -> Result<Box<dyn Rpc + Send + Sync>> {
    match chain {
        "ethereum" => {
            let endpoint = providers.ethereum.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let eth = ethereum::Ethereum::new(config)?;
            Ok(Box::new(eth))
        }
        "solana" => {
            let endpoint = providers.solana.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let sol = solana::Solana::new(config)?;
            Ok(Box::new(sol))
        }
        other => Err(anyhow!("Unsupported blockchain: {}. Supported: 'ethereum', 'solana'", other)),
    }
}

#[async_trait]
pub trait Rpc: Send + Sync {
    async fn fetch_latest_block(&self) -> Result<Option<BlockHeader>>;
}

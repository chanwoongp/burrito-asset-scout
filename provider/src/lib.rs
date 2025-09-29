pub mod ethereum;
pub mod solana;
pub mod types;
pub mod config;

pub use types::Config;
pub use ethereum::Ethereum;
pub use config::Providers;

use anyhow::{Result, anyhow};
use crate::types::BlockHeader;
use async_trait::async_trait;

#[derive(Debug)]
pub enum ProviderClient {
    Ethereum(Ethereum),
    Solana(solana::Solana),
}

pub fn new_client(chain: &str, providers: &Providers) -> Result<(String, ProviderClient)> {
    match chain {
        "ethereum" => {
            let endpoint = providers.ethereum.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let eth = Ethereum::new(config)?;
            Ok((endpoint, ProviderClient::Ethereum(eth)))
        }
        "solana" => {
            let endpoint = providers.solana.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let sol = solana::Solana::new(config)?;
            Ok((endpoint, ProviderClient::Solana(sol)))
        }
        other => Err(anyhow!("Unsupported blockchain: {}. Supported: 'ethereum', 'solana'", other)),
    }
}

pub fn new_rpc(chain: &str, providers: &Providers) -> Result<(String, Box<dyn Rpc + Send + Sync>)> {
    match chain {
        "ethereum" => {
            let endpoint = providers.ethereum.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let eth = Ethereum::new(config)?;
            Ok((endpoint, Box::new(eth)))
        }
        "solana" => {
            let endpoint = providers.solana.rpc.url.clone();
            let config = Config::new(endpoint.clone());
            let sol = solana::Solana::new(config)?;
            Ok((endpoint, Box::new(sol)))
        }
        other => Err(anyhow!("Unsupported blockchain: {}. Supported: 'ethereum', 'solana'", other)),
    }
}

#[async_trait]
pub trait Rpc: Send + Sync {
    async fn fetch_latest_block(&self) -> Result<Option<BlockHeader>>;
}

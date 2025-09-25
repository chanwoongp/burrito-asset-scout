pub mod ethereum;
pub mod types;

pub use ethereum::Ethereum;

use anyhow::{anyhow, Result};
use reqwest::Client;
use std::future::Future;

#[derive(Debug)]
pub struct BlockHeader {
    pub number: Option<u64>,
    pub hash: Option<String>,
}

pub trait Rpc {
    fn fetch_latest_block(&self, client: &Client, endpoint: &str) -> impl Future<Output = Result<Option<BlockHeader>>> + Send;
}

/// Fetch the latest block header for the specified chain.
/// Currently, supports: "ethereum".
pub async fn fetch_latest_block(
    client: &Client,
    endpoint: &str,
    chain: &str,
) -> Result<Option<BlockHeader>> {
    match chain {
        "ethereum" => ethereum::fetch_latest_block(client, endpoint).await,
        other => Err(anyhow!("Unsupported chain '{}'", other)),
    }
}

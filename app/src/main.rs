use anyhow::Result;
use provider::{Ethereum, Rpc};
use reqwest::Client;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Select blockchain via env var; default to ethereum for now.
    let chain = env::var("BLOCKCHAIN").unwrap_or_else(|_| "ethereum".to_string());

    // Endpoint can be configured via env var; falls back to the provided default for Ethereum.
    let endpoint = env::var("RPC_ENDPOINT").unwrap_or_else(|_| {
        "https://ethereum-mainnet.g.allthatnode.com/full/evm/cd69347a2bef449a96e00d2c38ae2f64".to_string()
    });

    let client = Client::builder().build()?;
    let ethereum = Ethereum;

    println!(
        "Starting periodic JSON-RPC calls for chain '{}' to {} (every 10s)...",
        chain, endpoint
    );

    loop {
        match ethereum.fetch_latest_block(&client, &endpoint).await {
            Ok(Some(block)) => {
                let num = block.number.unwrap_or_default();
                let hash = block.hash.unwrap_or_else(|| "<none>".to_string());
                println!("Latest block ({}): number={}, hash={}", chain, num, hash);
            }
            Ok(None) => {
                eprintln!("No latest block returned by provider ({})", chain);
            }
            Err(e) => {
                eprintln!("Error fetching latest block ({}): {e}", chain);
            }
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

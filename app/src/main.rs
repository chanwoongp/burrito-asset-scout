use anyhow::{Context, Result};
use provider::{Config, Ethereum, Rpc};
use serde::Deserialize;
use std::env;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct AppConfig {
    ethereum: EthereumConfig,
}

#[derive(Debug, Deserialize)]
struct EthereumConfig {
    rpc: RpcConfig,
}

#[derive(Debug, Deserialize)]
struct RpcConfig {
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Select blockchain via env var; default to ethereum for now.
    let chain = env::var("BLOCKCHAIN").unwrap_or_else(|_| "ethereum".to_string());

    // Read config from YAML file
    let config_content = fs::read_to_string("config_local.yml")
        .context("Failed to read config_local.yml")?;
    let app_config: AppConfig = serde_yaml::from_str(&config_content)
        .context("Failed to parse config_local.yml")?;

    let endpoint = app_config.ethereum.rpc.url;
    let config = Config::new(endpoint.clone());
    let ethereum = Ethereum::new(config)?;

    println!(
        "Starting periodic JSON-RPC calls for chain '{}' to {} (every 10s)...",
        chain, endpoint
    );

    loop {
        match ethereum.fetch_latest_block().await {
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

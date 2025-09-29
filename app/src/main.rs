use anyhow::{Context, Result};
use provider::{Config, Ethereum, Rpc};
use serde::Deserialize;
use std::env;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct AppConfig {
    provider: Providers,
}

#[derive(Debug, Deserialize)]
struct Providers {
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
    // Support both: positional arg (e.g., `app ethereum`) and flag form (e.g., `--chain=ethereum` or `--chain ethereum`).
    let args: Vec<String> = env::args().collect();

    // Parse chain from args
    let mut chain: Option<String> = None;
    let mut i = 1; // skip program name
    while i < args.len() {
        let arg = &args[i];
        if arg == "--chain" {
            if i + 1 < args.len() {
                chain = Some(args[i + 1].clone());
                i += 1; // consume value
            }
        } else if let Some(rest) = arg.strip_prefix("--chain=") {
            chain = Some(rest.to_string());
        } else if !arg.starts_with('-') && chain.is_none() {
            // fallback: first positional non-flag argument
            chain = Some(arg.clone());
        }
        i += 1;
    }

    let chain = match chain {
        Some(c) => c,
        None => {
            let bin = args.get(0).cloned().unwrap_or_else(|| "app".to_string());
            eprintln!(
                "Usage:\n  {bin} --chain <blockchain>\n  {bin} <blockchain>\n\nExample:\n  {bin} --chain=ethereum\n  {bin} ethereum\n\nSupported blockchains: ethereum"
            );
            return Ok(());
        }
    };

    if chain != "ethereum" {
        eprintln!("Unsupported blockchain: {}. Only 'ethereum' is supported currently.", chain);
        return Ok(());
    }

    // Read config from YAML file
    let config_content = fs::read_to_string("config_local.yml")
        .context("Failed to read config_local.yml")?;
    let app_config: AppConfig = serde_yaml::from_str(&config_content)
        .context("Failed to parse config_local.yml")?;

    let endpoint = app_config.provider.ethereum.rpc.url;
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

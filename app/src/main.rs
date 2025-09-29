use anyhow::Result;
use provider::{Config, Ethereum, Rpc};
use std::time::Duration;
use clap::Parser;

mod config;

#[derive(Debug, Parser)]
#[command(name = "burrito-asset-scout")]
struct Cli {
    #[arg(short, long, value_name = "CHAIN", help = "Blockchain to use (e.g., ethereum)", required = true)]
    chain: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line with clap (requires --chain/-c)
    let cli = Cli::parse();
    let chain = cli.chain;

    if chain != "ethereum" {
        eprintln!("Unsupported blockchain: {}. Only 'ethereum' is supported currently.", chain);
        return Ok(());
    }

    // Read config from YAML file via separate module
    let app_config = crate::config::load_app_config("config_local.yml")?;

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

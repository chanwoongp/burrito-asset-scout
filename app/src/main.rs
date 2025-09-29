use anyhow::Result;
use provider::{Config, Ethereum, Rpc};
use provider::solana::Solana;
use std::time::Duration;
use clap::Parser;

mod config;

#[derive(Debug, Parser)]
#[command(name = "burrito-asset-scout")]
struct Cli {
    #[arg(short, long, value_name = "CHAIN", help = "Blockchain to use (e.g., ethereum, solana)", required = true)]
    chain: String,
}

enum ProviderClient {
    Ethereum(Ethereum),
    Solana(Solana),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line with clap (requires --chain/-c)
    let cli = Cli::parse();
    let chain = cli.chain.to_lowercase();

    // Read config from YAML file via separate module
    let app_config = crate::config::load_app_config("config_local.yml")?;

    // Select endpoint and client by chain
    let (endpoint, client) = match chain.as_str() {
        "ethereum" => {
            let endpoint = app_config.provider.ethereum.rpc.url;
            let config = Config::new(endpoint.clone());
            let eth = Ethereum::new(config)?;
            (endpoint, ProviderClient::Ethereum(eth))
        }
        "solana" => {
            let endpoint = app_config.provider.solana.rpc.url;
            let config = Config::new(endpoint.clone());
            let sol = Solana::new(config)?;
            (endpoint, ProviderClient::Solana(sol))
        }
        other => {
            eprintln!("Unsupported blockchain: {}. Supported: 'ethereum', 'solana'", other);
            return Ok(());
        }
    };

    println!(
        "Starting periodic JSON-RPC calls for chain '{}' to {} (every 10s)...",
        chain, endpoint
    );

    loop {
        let res = match &client {
            ProviderClient::Ethereum(eth) => eth.fetch_latest_block().await,
            ProviderClient::Solana(sol) => sol.fetch_latest_block().await,
        };

        match res {
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

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

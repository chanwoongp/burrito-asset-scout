use anyhow::Result;
use provider::{new_provider, Chain};
use std::str::FromStr;
use std::time::Duration;
use clap::Parser;

mod config;

#[derive(Debug, Parser)]
#[command(name = "analyzer")]
struct Cli {
    #[arg(short, long, value_name = "CONFIG", help = "Configuration file path", required = true)]
    config: String,

    #[arg(long, value_name = "CHAIN", help = "Blockchain to use (e.g., ethereum, solana)", required = true)]
    chain: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config.to_lowercase();
    let fetcher_config = crate::config::load_config(&config_path)?;

    let chain_str = cli.chain.to_lowercase();
    let chain = match Chain::from_str(&chain_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    let _provider = match new_provider(chain, &fetcher_config.provider) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    loop {
        println!("Fetching latest block info...");

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}

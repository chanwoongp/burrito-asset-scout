use anyhow::Result;
use provider::{new_provider, Chain};
use std::str::FromStr;
use std::time::Duration;
use clap::Parser;

mod config;

#[derive(Debug, Parser)]
#[command(name = "fetcher")]
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
    let fetcher_config = config::load_config(&config_path)?;

    let chain_str = cli.chain.to_lowercase();
    let chain = match Chain::from_str(&chain_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    let provider = match new_provider(chain, &fetcher_config.provider) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    loop {
        let res_block_header = provider.fetch_latest_block_info().await;
        match res_block_header {
            Ok(Some(block)) => {
                let num = block.number;
                let hash = block.hash;
                println!("Latest block ({}): number={}, hash={}", chain_str, num, hash);
            }
            Ok(None) => {
                eprintln!("No latest block returned by provider ({})", chain_str);
            }
            Err(e) => {
                eprintln!("Error fetching latest block ({}): {e}", chain_str);
            }
        }

        let res_block_info = provider.fetch_block_data(100).await;
        match res_block_info {
            Ok(Some(block)) => {
                match block {
                    provider::types::AnyBlockData::Ethereum(b) => {
                        println!("Block 100: {:?}", b.block_header.hash);
                    }
                    provider::types::AnyBlockData::Solana(b) => {
                        println!("Block 100: {:?}", b.block_header.blockhash);
                    }
                }
            }
            Ok(None) => {
                eprintln!("No block 100 returned by provider ({})", chain_str);
            }
            Err(e) => {
                eprintln!("Error fetching block 100 ({}): {e}", chain_str);
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
use anyhow::Result;
use clap::Parser;
use provider::{Chain, new_provider};
use std::str::FromStr;
use std::time::Duration;

mod config;

#[derive(Debug, Parser)]
#[command(name = "fetcher")]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "CONFIG",
        help = "Configuration file path",
        required = true
    )]
    config: String,

    #[arg(
        long,
        value_name = "CHAIN",
        help = "Blockchain to use (e.g., ethereum, solana)",
        required = true
    )]
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
        let res_latest_block_header = provider.fetch_latest_block_info().await;
        let (block_number, block_hash) = match res_latest_block_header {
            Ok(Some(block)) => {
                let num = block.number;
                let hash = block.hash;
                println!(
                    "Latest block ({}): number={}, hash={}",
                    chain_str, num, hash
                );
                (num, hash)
            }
            Ok(None) => {
                eprintln!("No latest block returned by provider ({})", chain_str);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error fetching latest block ({}): {e}", chain_str);
                return Ok(());
            }
        };

        let res_block_info = provider.fetch_block_data(block_number).await;
        match res_block_info {
            Ok(Some(block)) => match block {
                provider::types::AnyBlockData::Ethereum(b) => {
                    println!("Block: {:?}", b);
                }
                provider::types::AnyBlockData::Solana(b) => {
                    println!("Block: {:?}", b);
                }
            },
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

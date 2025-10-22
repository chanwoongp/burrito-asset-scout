use anyhow::Result;
use provider::{new_provider, Chain};
use std::str::FromStr;
use std::time::Duration;
use clap::Parser;
use analyzer::{get_analyzer, AnalyzerType};

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

    let provider = match new_provider(chain, &fetcher_config.provider) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    loop {
        println!("Fetching latest block via provider for chain: {:?}", chain);

        match provider.fetch_latest_block_info().await {
            Ok(Some(latest)) => {
                match provider.fetch_block_data(latest.number).await {
                    Ok(Some(block)) => {
                        // TODO: chaining

                        if let Some(analyzer) = get_analyzer(AnalyzerType::CoinTransfer, chain) {
                            analyzer.analyze(&block);
                        } else {
                            eprintln!("No CoinTransfer analyzer available for chain: {:?}", chain);
                        }

                        if let Some(analyzer) = get_analyzer(AnalyzerType::TokenTransfer, chain) {
                            analyzer.analyze(&block);
                        } else {
                            eprintln!("No TokenTransfer analyzer available for chain: {:?}", chain);
                        }
                    }
                    Ok(None) => {
                        eprintln!("No block data returned for block number {}", latest.number);
                    }
                    Err(e) => {
                        eprintln!("Error fetching block data: {}", e);
                    }
                }
            }
            Ok(None) => {
                eprintln!("No latest block info available");
            }
            Err(e) => {
                eprintln!("Error fetching latest block info: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

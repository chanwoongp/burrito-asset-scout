use anyhow::Result;
use provider::{new_provider, Chain};
use std::str::FromStr;
use std::time::Duration;
use clap::Parser;
use analyzer::{get_analyzer, AnalyzerType};
use provider::types::{AnyBlockData, AnyBlockHeader, BlockData as GenericBlockData};
use provider::{ethereum, solana};

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

    // Build per-chain test data outside the loop and reuse inside
    let (ct_header, ct_data, tt_header, tt_data) = match chain {
        Chain::Ethereum => {
            let ct_header = AnyBlockHeader::Ethereum(ethereum::BlockHeader {
                hash: "0xdeadbeef".to_string(),
                parent_hash: "0xparent".to_string(),
                number: 1,
            });
            let ct_data = AnyBlockData::Ethereum(GenericBlockData::<ethereum::BlockData> {
                block_header: ethereum::BlockHeader {
                    hash: "0xdeadbeef".to_string(),
                    parent_hash: "0xparent".to_string(),
                    number: 1,
                },
                transactions: vec![],
            });
            let tt_header = AnyBlockHeader::Ethereum(ethereum::BlockHeader {
                hash: "0xdeadbeef".to_string(),
                parent_hash: "0xparent".to_string(),
                number: 2,
            });
            let tt_data = AnyBlockData::Ethereum(GenericBlockData::<ethereum::BlockData> {
                block_header: ethereum::BlockHeader {
                    hash: "0xdeadbeef".to_string(),
                    parent_hash: "0xparent".to_string(),
                    number: 2,
                },
                transactions: vec![],
            });
            (ct_header, ct_data, tt_header, tt_data)
        }
        Chain::Solana => {
            let ct_header = AnyBlockHeader::Solana(solana::BlockHeader {
                block_height: 222,
                block_time: 0,
                blockhash: "So11111111111111111111111111111111111111112".to_string(),
                parent_lot: 0,
                previous_blockhash: "So00000000000000000000000000000000000000000".to_string(),
            });
            let ct_data = AnyBlockData::Solana(GenericBlockData::<solana::BlockData> {
                block_header: solana::BlockHeader {
                    block_height: 222,
                    block_time: 0,
                    blockhash: "So11111111111111111111111111111111111111112".to_string(),
                    parent_lot: 0,
                    previous_blockhash: "So00000000000000000000000000000000000000000".to_string(),
                },
                transactions: vec![],
            });
            let tt_header = AnyBlockHeader::Solana(solana::BlockHeader {
                block_height: 333,
                block_time: 0,
                blockhash: "So33333333333333333333333333333333333333333".to_string(),
                parent_lot: 0,
                previous_blockhash: "So22222222222222222222222222222222222222222".to_string(),
            });
            let tt_data = AnyBlockData::Solana(GenericBlockData::<solana::BlockData> {
                block_header: solana::BlockHeader {
                    block_height: 333,
                    block_time: 0,
                    blockhash: "So33333333333333333333333333333333333333333".to_string(),
                    parent_lot: 0,
                    previous_blockhash: "So22222222222222222222222222222222222222222".to_string(),
                },
                transactions: vec![],
            });
            (ct_header, ct_data, tt_header, tt_data)
        }
    };

    loop {
        println!("Query data from database");

        tokio::time::sleep(Duration::from_secs(3)).await;

        if let Some(analyzer) = get_analyzer(AnalyzerType::CoinTransfer, chain) {
            analyzer.analyze(&ct_header, &ct_data);
        } else {
            eprintln!("No CoinTransfer analyzer available for chain: {:?}", chain);
        }

        if let Some(analyzer) = get_analyzer(AnalyzerType::TokenTransfer, chain) {
            analyzer.analyze(&tt_header, &tt_data);
        } else {
            eprintln!("No TokenTransfer analyzer available for chain: {:?}", chain);
        }
    }
}

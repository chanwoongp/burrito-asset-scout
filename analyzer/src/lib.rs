pub mod coin_transfer;
pub mod token_transfer;

use coin_transfer::ethereum::EthereumNativeCoinTransfer;
use coin_transfer::solana::SolanaNativeCoinTransfer;
use token_transfer::ethereum::EthereumTokenTransfer;
use token_transfer::solana::SolanaTokenTransfer;

pub trait Analyzer {
    fn analyze(&self);
}

pub fn get_analyzer(analyzer_type: &str, chain: &str) -> Option<Box<dyn Analyzer>> {
    match analyzer_type {
        "coin_transfer" => match chain {
            "ethereum" => Some(Box::new(EthereumNativeCoinTransfer {})),
            "solana" => Some(Box::new(SolanaNativeCoinTransfer {})),
            _ => None,
        },
        "token_transfer" => match chain {
            "ethereum" => Some(Box::new(EthereumTokenTransfer {})),
            "solana" => Some(Box::new(SolanaTokenTransfer {})),
            _ => None,
        },
        _ => None,
    }
}
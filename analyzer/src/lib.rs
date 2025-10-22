pub mod coin_transfer;
pub mod token_transfer;

use provider::Chain;
use provider::types::AnyBlockData;
use coin_transfer::ethereum::EthereumCoinTransfer;
use coin_transfer::solana::SolanaCoinTransfer;
use token_transfer::ethereum::EthereumTokenTransfer;
use token_transfer::solana::SolanaTokenTransfer;

pub trait Analyzer {
    fn analyze(&self, data: &AnyBlockData);
}

pub enum AnalyzerType {
    CoinTransfer,
    TokenTransfer,
}

pub fn get_analyzer(analyzer_type: AnalyzerType, chain: Chain) -> Option<Box<dyn Analyzer>> {
    match analyzer_type {
        AnalyzerType::CoinTransfer => match chain {
            Chain::Ethereum => Some(Box::new(EthereumCoinTransfer {})),
            Chain::Solana => Some(Box::new(SolanaCoinTransfer {})),
        },
        AnalyzerType::TokenTransfer => match chain {
            Chain::Ethereum => Some(Box::new(EthereumTokenTransfer {})),
            Chain::Solana => Some(Box::new(SolanaTokenTransfer {})),
        },
    }
}

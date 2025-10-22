use crate::Analyzer;
use provider::types::{AnyBlockData, AnyBlockHeader};
use provider::ethereum;

pub struct EthereumCoinTransfer;

impl Analyzer for EthereumCoinTransfer {
    fn analyze(&self, header: &AnyBlockHeader, data: &AnyBlockData) {
        if let (AnyBlockHeader::Ethereum(h), AnyBlockData::Ethereum(d)) = (header, data) {
            println!(
                "ethereum coin transfer analyzer: block #{}, hash {} (txs: {})",
                h.number,
                h.hash,
                d.transactions.len()
            );
        } else {
            eprintln!("EthereumCoinTransfer received non-ethereum data");
        }
    }
}

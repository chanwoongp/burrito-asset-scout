use crate::Analyzer;
use provider::types::{AnyBlockData, AnyBlockHeader};

pub struct EthereumTokenTransfer;

impl Analyzer for EthereumTokenTransfer {
    fn analyze(&self, header: AnyBlockHeader, data: AnyBlockData) {
        if let (AnyBlockHeader::Ethereum(h), AnyBlockData::Ethereum(d)) = (header, data) {
            println!(
                "ethereum token transfer analyzer: block #{}, hash {} (txs: {})",
                h.number,
                h.hash,
                d.transactions.len()
            );
        } else {
            eprintln!("EthereumTokenTransfer received non-ethereum data");
        }
    }
}

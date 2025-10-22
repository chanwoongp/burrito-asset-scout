use crate::Analyzer;
use provider::types::AnyBlockData;

pub struct EthereumCoinTransfer;

impl Analyzer for EthereumCoinTransfer {
    fn analyze(&self, data: &AnyBlockData) {
        if let AnyBlockData::Ethereum(d) = data {
            let h = &d.block_header;
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

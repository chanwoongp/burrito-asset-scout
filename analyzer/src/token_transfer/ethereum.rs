use crate::Analyzer;
use provider::types::AnyBlockData;

pub struct EthereumTokenTransfer;

impl Analyzer for EthereumTokenTransfer {
    fn analyze(&self, data: &AnyBlockData) {
        if let AnyBlockData::Ethereum(d) = data {
            let h = &d.block_header;
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

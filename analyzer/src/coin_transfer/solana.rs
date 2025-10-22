use crate::Analyzer;
use provider::types::{AnyBlockData, AnyBlockHeader};

pub struct SolanaCoinTransfer;

impl Analyzer for SolanaCoinTransfer {
    fn analyze(&self, header: AnyBlockHeader, data: AnyBlockData) {
        if let (AnyBlockHeader::Solana(h), AnyBlockData::Solana(d)) = (header, data) {
            println!(
                "solana coin transfer analyzer: blockhash {}, height {} (txs: {})",
                h.blockhash,
                h.block_height,
                d.transactions.len()
            );
        } else {
            eprintln!("SolanaCoinTransfer received non-solana data");
        }
    }
}

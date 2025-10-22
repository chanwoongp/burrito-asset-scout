use crate::Analyzer;
use provider::types::{AnyBlockData, AnyBlockHeader};

pub struct SolanaTokenTransfer;

impl Analyzer for SolanaTokenTransfer {
    fn analyze(&self, header: AnyBlockHeader, data: AnyBlockData) {
        if let (AnyBlockHeader::Solana(h), AnyBlockData::Solana(d)) = (header, data) {
            println!(
                "solana token transfer analyzer: blockhash {}, height {} (txs: {})",
                h.blockhash,
                h.block_height,
                d.transactions.len()
            );
        } else {
            eprintln!("SolanaTokenTransfer received non-solana data");
        }
    }
}

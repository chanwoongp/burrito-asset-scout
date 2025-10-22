use crate::Analyzer;
use provider::types::AnyBlockData;

pub struct SolanaTokenTransfer;

impl Analyzer for SolanaTokenTransfer {
    fn analyze(&self, data: &AnyBlockData) {
        if let AnyBlockData::Solana(d) = data {
            let h = &d.block_header;
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

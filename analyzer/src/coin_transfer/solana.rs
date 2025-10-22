use crate::Analyzer;
use provider::types::AnyBlockData;

pub struct SolanaCoinTransfer;

impl Analyzer for SolanaCoinTransfer {
    fn analyze(&self, data: &AnyBlockData) {
        if let AnyBlockData::Solana(d) = data {
            let h = &d.block_header;
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

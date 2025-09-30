use crate::Analyzer;

pub struct SolanaTokenTransfer;

impl Analyzer for SolanaTokenTransfer {
    fn analyze(&self) {
        println!("hello, world from solana token transfer analyzer");
    }
}

use crate::Analyzer;

pub struct SolanaCoinTransfer;

impl Analyzer for SolanaCoinTransfer {
    fn analyze(&self) {
        println!("hello, world from solana coin transfer analyzer");
    }
}

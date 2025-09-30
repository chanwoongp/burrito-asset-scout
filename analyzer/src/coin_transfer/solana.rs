use crate::Analyzer;

pub struct SolanaNativeCoinTransfer;

impl Analyzer for SolanaNativeCoinTransfer {
    fn analyze(&self) {
        println!("hello, world from solana native coin transfer analyzer");
    }
}

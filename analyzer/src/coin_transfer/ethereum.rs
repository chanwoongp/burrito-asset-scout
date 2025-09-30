use crate::Analyzer;

pub struct EthereumCoinTransfer;

impl Analyzer for EthereumCoinTransfer {
    fn analyze(&self) {
        println!("hello, world from ethereum coin transfer analyzer");
    }
}

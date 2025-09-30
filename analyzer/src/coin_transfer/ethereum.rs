use crate::Analyzer;

pub struct EthereumNativeCoinTransfer;

impl Analyzer for EthereumNativeCoinTransfer {
    fn analyze(&self) {
        println!("hello, world from ethereum coin transfer analyzer");
    }
}

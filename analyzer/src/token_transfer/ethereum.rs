use crate::Analyzer;

pub struct EthereumTokenTransfer;

impl Analyzer for EthereumTokenTransfer {
    fn analyze(&self) {
        println!("hello, world from ethereum token transfer analyzer");
    }
}

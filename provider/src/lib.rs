pub mod ethereum;
pub mod types;

pub use ethereum::Ethereum;

use anyhow::{Result};
use std::future::Future;

#[derive(Debug)]
pub struct BlockHeader {
    pub number: Option<u64>,
    pub hash: Option<String>,
}

pub trait Rpc {
    fn fetch_latest_block(&self, endpoint: &str) -> impl Future<Output = Result<Option<BlockHeader>>> + Send;
}

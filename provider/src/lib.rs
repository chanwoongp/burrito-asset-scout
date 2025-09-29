pub mod ethereum;
pub mod types;

pub use types::Config;
pub use ethereum::Ethereum;

use anyhow::Result;
use std::future::Future;
use crate::types::BlockHeader;

pub trait Rpc {
    fn fetch_latest_block(&self) -> impl Future<Output = Result<Option<BlockHeader>>> + Send;
}

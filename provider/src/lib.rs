pub mod ethereum;
pub mod solana;
pub mod types;
pub mod config;

pub use types::Config;
pub use ethereum::Ethereum;
pub use config::Providers;

use anyhow::Result;
use std::future::Future;
use crate::types::BlockHeader;

pub trait Rpc {
    fn fetch_latest_block(&self) -> impl Future<Output = Result<Option<BlockHeader>>> + Send;
}

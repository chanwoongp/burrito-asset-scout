use serde::Deserialize;
use std::fmt;
use std::str::FromStr;
use anyhow::anyhow;
use crate::{ethereum, solana};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Ethereum,
    Solana,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Chain::Ethereum => "ethereum",
            Chain::Solana => "solana",
        })
    }
}

impl FromStr for Chain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" => Ok(Chain::Ethereum),
            "solana" => Ok(Chain::Solana),
            other => Err(anyhow!("Unsupported chain: '{}'", other)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
}

impl Config {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[derive(Debug)]
pub struct LatestBlockInfo {
    pub number: Option<u64>,
    pub hash: Option<String>,
}

pub enum AnyBlockHeader {
    Ethereum(ethereum::BlockHeader),
    Solana(solana::BlockHeader),
}

pub enum AnyTransaction {
    Ethereum(ethereum::Transaction),
    Solana(solana::Transaction),
}

pub trait IBlockData {
    type BlockHeader;
    type Transaction;
}

pub struct BlockData<C: IBlockData> {
    pub block_header: C::BlockHeader,
    pub transactions: Vec<C::Transaction>,
}

pub enum AnyBlockData {
    Ethereum(BlockData<ethereum::BlockData>),
    Solana(BlockData<solana::BlockData>),
}
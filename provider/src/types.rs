use serde::Deserialize;
use std::fmt;
use std::str::FromStr;
use anyhow::anyhow;
use crate::{ethereum, solana, types};

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

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    #[allow(unused)]
    pub jsonrpc: Option<String>,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
    #[allow(unused)]
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug)]
pub struct BlockHeader {
    pub number: Option<u64>,
    pub hash: Option<String>,
}

pub trait IBlock {
    type BlockMetadata;
    type Transaction;
}

pub struct Block<C: IBlock> {
    pub metadata: C::BlockMetadata,
    pub transactions: Vec<C::Transaction>,
}

pub enum AnyBlock {
    Ethereum(Block<ethereum::Block>),
    Solana(Block<solana::Block>),
}
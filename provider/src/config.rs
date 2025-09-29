use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rpc {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChainProvider {
    pub rpc: Rpc,
}

// Aggregate configuration for all supported providers.
#[derive(Debug, Deserialize, Clone)]
pub struct Providers {
    pub ethereum: ChainProvider,
}

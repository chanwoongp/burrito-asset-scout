use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ethereum {
    pub rpc: Rpc,
}

#[derive(Debug, Deserialize)]
pub struct Rpc {
    pub url: String,
}

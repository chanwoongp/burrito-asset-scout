use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub provider: Providers,
}

#[derive(Debug, Deserialize)]
pub struct Providers {
    pub ethereum: EthereumConfig,
}

#[derive(Debug, Deserialize)]
pub struct EthereumConfig {
    pub rpc: RpcConfig,
}

#[derive(Debug, Deserialize)]
pub struct RpcConfig {
    pub url: String,
}

pub fn load_app_config(path: &str) -> Result<AppConfig> {
    let config_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    let app_config: AppConfig = serde_yaml::from_str(&config_content)
        .with_context(|| format!("Failed to parse {}", path))?;
    Ok(app_config)
}

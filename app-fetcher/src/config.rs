use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use provider::config::Providers;

#[derive(Debug, Deserialize)]
pub struct FetcherConfig {
    pub provider: Providers,
}

pub fn load_config(path: &str) -> Result<FetcherConfig> {
    let config_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    let config: FetcherConfig = serde_yaml::from_str(&config_content)
        .with_context(|| format!("Failed to parse {}", path))?;
    Ok(config)
}

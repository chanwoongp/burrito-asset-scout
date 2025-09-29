use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use provider::config::Providers;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub provider: Providers,
}

pub fn load_app_config(path: &str) -> Result<AppConfig> {
    let config_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    let app_config: AppConfig = serde_yaml::from_str(&config_content)
        .with_context(|| format!("Failed to parse {}", path))?;
    Ok(app_config)
}

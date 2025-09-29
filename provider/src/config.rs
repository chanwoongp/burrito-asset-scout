use serde::Deserialize;

// Aggregate configuration for all supported providers.
#[derive(Debug, Deserialize)]
pub struct Providers {
    pub ethereum: crate::ethereum::config::Ethereum,
}

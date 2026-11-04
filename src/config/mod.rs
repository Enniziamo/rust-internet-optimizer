use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub proxy: ProxyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub listen: SocketAddr,
    pub allowed_connect_ports: Vec<u16>,
    pub max_connections: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy: ProxyConfig::default(),
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            listen: "127.0.0.1:8080".parse().unwrap(),
            allowed_connect_ports: vec![443],
            max_connections: 128,
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Could not read configuration file")?;

        let config: Self =
            toml::from_str(&content).context("Invalid TOML configuration")?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.proxy.listen.ip().is_loopback(),
            "Initial proxy release only supports localhost binding"
        );

        anyhow::ensure!(
            self.proxy.max_connections > 0,
            "max_connections must be greater than zero"
        );

        anyhow::ensure!(
            !self.proxy.allowed_connect_ports.is_empty(),
            "At least one CONNECT port must be configured"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_localhost_only() {
        let config = Config::default();

        assert!(config.proxy.listen.ip().is_loopback());
        assert!(!config.proxy.enabled);
        assert_eq!(config.proxy.allowed_connect_ports, vec![443]);
    }

    #[test]
    fn non_loopback_binding_is_rejected() {
        let mut config = Config::default();

        config.proxy.listen = "0.0.0.0:8080".parse().unwrap();

        assert!(config.validate().is_err());
    }
}
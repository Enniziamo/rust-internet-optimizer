mod cli;
mod config;
mod network;
mod optimizer;
mod proxy;

use anyhow::{bail, Result};
use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = cli::Args::parse();

    println!("\n==================================");
    println!("     RIO - INTERNET OPTIMIZER");
    println!("==================================\n");

    match args.command {
        cli::Command::Check { host } => {
            network::analyzer::check(&host).await?;
        }

        cli::Command::Dns { host } => {
            network::dns::test(&host).await?;
        }

        cli::Command::Tcp { host, port } => {
            network::ping::test(&host, port).await?;
        }

        cli::Command::Speed { url } => {
            network::speed::test(&url).await?;
        }

        cli::Command::Optimize => {
            optimizer::recommendations::run().await?;
        }

        cli::Command::Proxy { command } => match command {
            cli::ProxyCommand::Start { config: path } => {
                let config = config::Config::load(&path)?;

                if !config.proxy.enabled {
                    bail!("Proxy is disabled in configuration");
                }

                proxy::run(config.proxy).await?;
            }
        },

        cli::Command::Config { command } => match command {
            cli::ConfigCommand::Check { path } => {
                config::Config::load(&path)?;

                println!("Configuration is valid.");
            }
        },
    }

    Ok(())
}

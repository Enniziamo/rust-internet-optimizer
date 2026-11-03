
mod cli;
mod network;
mod optimizer;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
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
    }

    Ok(())
}

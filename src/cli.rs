use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rio",
    version,
    about = "Rust Internet Optimizer",
    long_about = "Network diagnostics, optimization and forward proxy toolkit."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// General connectivity check
    Check {
        #[arg(default_value = "example.com")]
        host: String,
    },

    /// DNS resolution test
    Dns {
        #[arg(default_value = "example.com")]
        host: String,
    },

    /// TCP connection latency test
    Tcp {
        #[arg(default_value = "google.com")]
        host: String,

        #[arg(short, long, default_value_t = 443)]
        port: u16,
    },

    /// HTTP download speed test
    Speed { url: String },

    /// Display network optimization recommendations
    Optimize,

    /// Manage the forward proxy
    Proxy {
        #[command(subcommand)]
        command: ProxyCommand,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Subcommand)]
pub enum ProxyCommand {
    /// Start the forward proxy
    Start {
        #[arg(short, long, default_value = "config/default.toml")]
        config: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Validate a configuration file
    Check {
        #[arg(short, long, default_value = "config/default.toml")]
        path: PathBuf,
    },
}

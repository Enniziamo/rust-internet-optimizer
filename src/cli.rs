
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rio",
    version,
    about = "Rust Internet Optimizer"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Genel bağlantı kontrolü
    Check {
        #[arg(default_value = "example.com")]
        host: String,
    },

    /// DNS çözümleme testi
    Dns {
        #[arg(default_value = "example.com")]
        host: String,
    },

    /// TCP bağlantı gecikmesi testi
    Tcp {
        #[arg(default_value = "google.com")]
        host: String,

        #[arg(short, long, default_value_t = 443)]
        port: u16,
    },

    /// HTTP indirme testi
    Speed {
        url: String,
    },

    /// Güvenli optimizasyon önerileri
    Optimize,
}

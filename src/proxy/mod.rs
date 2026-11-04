pub mod connect;

use anyhow::Result;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::Semaphore,
};
use tracing::{error, info};

use crate::config::ProxyConfig;

pub async fn run(config: ProxyConfig) -> Result<()> {
    let listener: TcpListener =
        TcpListener::bind(config.listen).await?;

    let semaphore =
        Arc::new(Semaphore::new(config.max_connections));

    info!("RIO proxy listening on {}", config.listen);

    loop {
        let (stream, peer) = listener.accept().await?;

        if !peer.ip().is_loopback() {
            continue;
        }

        let permit = match semaphore.clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                info!("Connection limit reached: {}", peer);
                continue;
            }
        };

        let allowed_ports = config.allowed_connect_ports.clone();

        tokio::spawn(async move {
            let _permit = permit;

            if let Err(err) =
                connect::handle(stream, allowed_ports).await
            {
                error!("Proxy connection error: {err:#}");
            }
        });
    }
}
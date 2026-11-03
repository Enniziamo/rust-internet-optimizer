
use anyhow::{Context, Result};
use std::time::Instant;
use tokio::net::lookup_host;

pub async fn test(host: &str) -> Result<()> {
    println!("[DNS] Host: {host}");

    let start = Instant::now();

    let addresses = lookup_host((host, 443))
        .await
        .context("DNS çözümlemesi başarısız")?;

    let results: Vec<_> = addresses.collect();

    let elapsed = start.elapsed();

    if results.is_empty() {
        println!("Adres bulunamadı.");
    } else {
        for address in results {
            println!("IP: {address}");
        }

        println!("DNS süresi: {:.2} ms", elapsed.as_secs_f64() * 1000.0);
    }

    Ok(())
}

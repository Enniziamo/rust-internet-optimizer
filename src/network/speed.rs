
use anyhow::{Context, Result};
use futures_util::StreamExt;
use std::time::Instant;

pub async fn test(url: &str) -> Result<()> {
    const MAX_BYTES: u64 = 10 * 1024 * 1024;

    println!("[SPEED TEST]");
    println!("URL: {url}");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let start = Instant::now();

    let response = client
        .get(url)
        .send()
        .await
        .context("HTTP bağlantısı başarısız")?
        .error_for_status()?;

    let mut stream = response.bytes_stream();
    let mut total_bytes = 0_u64;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("İndirme sırasında hata")?;

        total_bytes += chunk.len() as u64;

        if total_bytes >= MAX_BYTES {
            total_bytes = MAX_BYTES;
            break;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();

    if elapsed <= 0.0 {
        anyhow::bail!("Ölçüm süresi geçersiz.");
    }

    let mbps = (total_bytes as f64 * 8.0)
        / elapsed
        / 1_000_000.0;

    let mbytes = total_bytes as f64 / 1_000_000.0;

    println!("İndirilen: {:.2} MB", mbytes);
    println!("Süre: {:.2} saniye", elapsed);
    println!("Hız: {:.2} Mbps", mbps);

    Ok(())
}

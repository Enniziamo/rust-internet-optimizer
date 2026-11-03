
use anyhow::Result;

pub async fn check(host: &str) -> Result<()> {
    println!("RIO NETWORK ANALYSIS");
    println!("--------------------");

    println!("\n1. DNS ANALYSIS");
    super::dns::test(host).await?;

    println!("\n2. TCP LATENCY");
    super::ping::test(host, 443).await?;

    println!("\nAnaliz tamamlandı.");

    Ok(())
}

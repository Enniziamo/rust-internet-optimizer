use anyhow::Result;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn test(host: &str, port: u16) -> Result<()> {
    println!("[TCP] {host}:{port}");

    for attempt in 1..=5 {
        let start = Instant::now();

        let result = timeout(Duration::from_secs(5), TcpStream::connect((host, port))).await;

        match result {
            Ok(Ok(_stream)) => {
                let elapsed = start.elapsed();

                println!("Deneme {attempt}: {:.2} ms", elapsed.as_secs_f64() * 1000.0);
            }

            Ok(Err(error)) => {
                println!("Deneme {attempt}: Bağlantı hatası: {error}");
            }

            Err(_) => {
                println!("Deneme {attempt}: Zaman aşımı");
            }
        }
    }

    Ok(())
}

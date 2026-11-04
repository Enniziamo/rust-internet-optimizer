use anyhow::{bail, Context, Result};
use tokio::{
    io::{
        copy_bidirectional, AsyncReadExt, AsyncWriteExt,
    },
    net::TcpStream,
};

const MAX_HEADER_SIZE: usize = 8192;

pub async fn handle(
    mut client: TcpStream,
    allowed_ports: Vec<u16>,
) -> Result<()> {
    let mut request = Vec::new();

    loop {
        let mut byte = [0u8; 1];

        let count = client.read(&mut byte).await?;

        if count == 0 {
            return Ok(());
        }

        request.push(byte[0]);

        anyhow::ensure!(
            request.len() <= MAX_HEADER_SIZE,
            "Request header too large"
        );

        if request.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    let request_text = std::str::from_utf8(&request)
        .context("Invalid HTTP request encoding")?;

    let request_line = request_text
        .lines()
        .next()
        .context("Missing HTTP request line")?;

    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("");
    let authority = parts.next().unwrap_or("");
    let version = parts.next().unwrap_or("");

    if method != "CONNECT" || version != "HTTP/1.1" {
        client
            .write_all(
                b"HTTP/1.1 405 Method Not Allowed\r\n\
                  Connection: close\r\n\
                  Content-Length: 0\r\n\r\n",
            )
            .await?;

        return Ok(());
    }

    let (host, port) = parse_authority(authority)?;

    if !allowed_ports.contains(&port) {
        client
            .write_all(
                b"HTTP/1.1 403 Forbidden\r\n\
                  Connection: close\r\n\
                  Content-Length: 0\r\n\r\n",
            )
            .await?;

        return Ok(());
    }

    let mut upstream = match TcpStream::connect((host.as_str(), port)).await {
        Ok(stream) => stream,
        Err(_) => {
            client
                .write_all(
                    b"HTTP/1.1 502 Bad Gateway\r\n\
                      Connection: close\r\n\
                      Content-Length: 0\r\n\r\n",
                )
                .await?;

            return Ok(());
        }
    };

    client
        .write_all(
            b"HTTP/1.1 200 Connection Established\r\n\r\n",
        )
        .await?;

    let (client_to_upstream, upstream_to_client) =
        copy_bidirectional(&mut client, &mut upstream).await?;

    tracing::debug!(
        uploaded = client_to_upstream,
        downloaded = upstream_to_client,
        "CONNECT tunnel closed"
    );

    Ok(())
}

fn parse_authority(authority: &str) -> Result<(String, u16)> {
    let (host, port) = if let Some(rest) = authority.strip_prefix('[') {
        let (host, suffix) = rest
            .split_once(']')
            .context("Invalid IPv6 authority")?;

        let port = suffix
            .strip_prefix(':')
            .context("Missing port")?;

        (host.to_string(), port)
    } else {
        let (host, port) = authority
            .rsplit_once(':')
            .context("Missing target port")?;

        (host.to_string(), port)
    };

    if host.is_empty() || host.contains('/') || host.contains('@') {
        bail!("Invalid target hostname");
    }

    let port: u16 = port.parse().context("Invalid target port")?;

    if port == 0 {
        bail!("Invalid target port");
    }

    Ok((host, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_domain_authority() {
        let result = parse_authority("example.com:443").unwrap();

        assert_eq!(result.0, "example.com");
        assert_eq!(result.1, 443);
    }

    #[test]
    fn parses_ipv6_authority() {
        let result = parse_authority("[::1]:443").unwrap();

        assert_eq!(result.0, "::1");
        assert_eq!(result.1, 443);
    }

    #[test]
    fn rejects_missing_port() {
        assert!(parse_authority("example.com").is_err());
    }

    #[test]
    fn rejects_zero_port() {
        assert!(parse_authority("example.com:0").is_err());
    }
}
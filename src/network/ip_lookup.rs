use anyhow::{anyhow, Result};
use std::time::Duration;

const ENDPOINTS: &[&str] = &[
    "https://api.ipify.org",
    "https://ifconfig.me/ip",
    "https://icanhazip.com",
    "https://checkip.amazonaws.com",
];

pub async fn fetch_public_ip(socks_port: Option<u16>) -> Result<String> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("Onionymous/1.0 (+https://onionymous.app)");

    if let Some(port) = socks_port {
        let url = format!("socks5h://127.0.0.1:{}", port);
        let proxy = reqwest::Proxy::all(&url)?;
        builder = builder.proxy(proxy);
    }

    let client = builder.build()?;

    let mut last_err: Option<anyhow::Error> = None;
    for endpoint in ENDPOINTS {
        match client.get(*endpoint).send().await {
            Ok(resp) if resp.status().is_success() => match resp.text().await {
                Ok(text) => {
                    let ip = text.trim().to_string();
                    if !ip.is_empty() {
                        return Ok(ip);
                    }
                    last_err = Some(anyhow!("empty response from {}", endpoint));
                }
                Err(e) => last_err = Some(e.into()),
            },
            Ok(resp) => {
                last_err = Some(anyhow!("{} returned {}", endpoint, resp.status()));
            }
            Err(e) => {
                last_err = Some(e.into());
            }
        }
    }
    Err(last_err.unwrap_or_else(|| anyhow!("All IP endpoints failed")))
}

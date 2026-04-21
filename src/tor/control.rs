use anyhow::{anyhow, bail, Context, Result};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct TorControl {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
}

impl TorControl {
    pub async fn connect(control_port: u16, cookie_path: &Path) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", control_port);
        let stream = TcpStream::connect(&addr)
            .await
            .with_context(|| format!("Connecting to Tor control port at {addr}"))?;

        let (read_half, write_half) = stream.into_split();
        let mut ctrl = Self {
            reader: BufReader::new(read_half),
            writer: write_half,
        };

        ctrl.authenticate_cookie(cookie_path).await?;
        Ok(ctrl)
    }

    async fn authenticate_cookie(&mut self, cookie_path: &Path) -> Result<()> {
        let cookie = tokio::fs::read(cookie_path)
            .await
            .with_context(|| format!("Reading cookie file at {}", cookie_path.display()))?;
        let hex = cookie.iter().map(|b| format!("{:02X}", b)).collect::<String>();
        let response = self.send_command(&format!("AUTHENTICATE {hex}")).await?;
        if !response.starts_with("250") {
            bail!("Tor AUTHENTICATE failed: {}", response);
        }
        Ok(())
    }

    pub async fn send_command(&mut self, cmd: &str) -> Result<String> {
        let line = format!("{}\r\n", cmd);
        self.writer.write_all(line.as_bytes()).await?;
        self.writer.flush().await?;

        let mut buf = String::new();
        loop {
            let mut line = String::new();
            let n = self.reader.read_line(&mut line).await?;
            if n == 0 {
                bail!("Tor control connection closed while waiting for reply");
            }
            let trimmed = line.trim_end().to_string();
            buf.push_str(&trimmed);
            buf.push('\n');

            if trimmed.len() >= 4 {
                let sep_ch = trimmed.as_bytes()[3] as char;
                let is_final = sep_ch == ' ';
                if is_final {
                    break;
                }
            }
        }
        Ok(buf)
    }

    pub async fn new_identity(&mut self) -> Result<()> {
        let reply = self.send_command("SIGNAL NEWNYM").await?;
        if !reply.starts_with("250") {
            bail!("NEWNYM failed: {reply}");
        }
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        let _ = self.send_command("SIGNAL SHUTDOWN").await?;
        Ok(())
    }

    pub async fn subscribe_events(&mut self) -> Result<()> {
        let reply = self.send_command("SETEVENTS BW STATUS_CLIENT").await?;
        if !reply.starts_with("250") {
            bail!("SETEVENTS failed: {reply}");
        }
        Ok(())
    }

    pub async fn read_event(&mut self) -> Result<Option<String>> {
        let mut line = String::new();
        let n = self.reader.read_line(&mut line).await?;
        if n == 0 {
            return Ok(None);
        }
        Ok(Some(line.trim_end().to_string()))
    }

    pub async fn get_info(&mut self, key: &str) -> Result<String> {
        let reply = self.send_command(&format!("GETINFO {key}")).await?;
        for line in reply.lines() {
            if let Some(rest) = line.strip_prefix("250-") {
                if let Some((_, val)) = rest.split_once('=') {
                    return Ok(val.to_string());
                }
            }
        }
        Err(anyhow!("GETINFO {} returned no value: {}", key, reply))
    }
}

pub fn parse_bw_event(line: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 && parts[0] == "650" && parts[1] == "BW" {
        let read = parts[2].parse().ok()?;
        let written = parts[3].parse().ok()?;
        Some((read, written))
    } else {
        None
    }
}


pub fn parse_bootstrap_event(line: &str) -> Option<(u8, String)> {
    if !line.contains("BOOTSTRAP") {
        return None;
    }
    let mut progress: Option<u8> = None;
    let mut summary: Option<String> = None;

    let mut chars = line.chars().peekable();
    let mut buf = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut in_quotes = false;
    while let Some(c) = chars.next() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !buf.is_empty() {
                    tokens.push(std::mem::take(&mut buf));
                }
            }
            _ => buf.push(c),
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    for token in tokens {
        if let Some(val) = token.strip_prefix("PROGRESS=") {
            progress = val.parse().ok();
        } else if let Some(val) = token.strip_prefix("SUMMARY=") {
            summary = Some(val.to_string());
        }
    }
    match (progress, summary) {
        (Some(p), Some(s)) => Some((p, s)),
        (Some(p), None) => Some((p, String::new())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bw() {
        let line = "650 BW 1024 2048";
        assert_eq!(parse_bw_event(line), Some((1024, 2048)));
    }

    #[test]
    fn parse_bootstrap() {
        let line = r#"650 STATUS_CLIENT NOTICE BOOTSTRAP PROGRESS=30 TAG=loading_keys SUMMARY=Loading authority key certs"#;
        let parsed = parse_bootstrap_event(line).unwrap();
        assert_eq!(parsed.0, 30);
        assert!(parsed.1.contains("Loading"));
    }
}

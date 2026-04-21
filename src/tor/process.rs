use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use crate::core::state::{LogEntry, LogLevel, LogSource, SharedState};

pub struct TorProcess {
    child: Arc<Mutex<Option<Child>>>,
    binary: PathBuf,
}

impl TorProcess {
    pub fn new(binary: PathBuf) -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            binary,
        }
    }

    pub fn locate_binary() -> Result<PathBuf> {
        if let Ok(p) = crate::resources::tor_binary_path() {
            if p.exists() {
                log::info!("Using extracted Tor binary at {}", p.display());
                return Ok(p);
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let bundled = parent.join("resources").join("tor");
                #[cfg(windows)]
                let bundled = bundled.join("tor.exe");
                #[cfg(not(windows))]
                let bundled = bundled.join("tor");

                if bundled.exists() {
                    log::info!("Using bundled Tor binary at {}", bundled.display());
                    return Ok(bundled);
                }
            }
        }

        which::which("tor")
            .context("Could not find `tor` binary. Rebuild with `--features embedded-tor`, \
                      drop tor.exe in `resources/tor/`, or install Tor system-wide.")
    }

    pub async fn start(&self, torrc_path: &Path, state: SharedState) -> Result<()> {
        let mut lock = self.child.lock().await;
        if lock.is_some() {
            bail!("Tor process is already running");
        }

        log::info!("Starting Tor with torrc {}", torrc_path.display());

        let mut cmd = Command::new(&self.binary);
        cmd.arg("-f")
            .arg(torrc_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .kill_on_drop(true);

        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Spawning {}", self.binary.display()))?;

        if let Some(stdout) = child.stdout.take() {
            let state_clone = state.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    push_tor_log_line(&state_clone, &line);
                }
            });
        }
        if let Some(stderr) = child.stderr.take() {
            let state_clone = state.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    push_tor_log_line(&state_clone, &line);
                }
            });
        }

        *lock = Some(child);
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut lock = self.child.lock().await;
        if let Some(mut child) = lock.take() {
            if let Err(e) = child.start_kill() {
                log::warn!("start_kill failed: {}", e);
            }
            match tokio::time::timeout(std::time::Duration::from_secs(5), child.wait()).await {
                Ok(Ok(status)) => log::info!("Tor exited with {}", status),
                Ok(Err(e)) => log::warn!("Waiting for Tor failed: {}", e),
                Err(_) => {
                    log::warn!("Tor did not exit within 5s, forcing kill");
                    let _ = child.kill().await;
                }
            }
        }
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        let mut lock = self.child.lock().await;
        if let Some(child) = lock.as_mut() {
            match child.try_wait() {
                Ok(None) => true,
                Ok(Some(status)) => {
                    log::info!("Tor process exited with {}", status);
                    *lock = None;
                    false
                }
                Err(e) => {
                    log::warn!("try_wait on Tor failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}

fn push_tor_log_line(state: &SharedState, line: &str) {
    let level = if line.contains("[err]") || line.contains("[error]") {
        LogLevel::Error
    } else if line.contains("[warn]") {
        LogLevel::Warn
    } else if line.contains("[notice]") {
        LogLevel::Notice
    } else if line.contains("[info]") {
        LogLevel::Info
    } else if line.contains("[debug]") {
        LogLevel::Debug
    } else {
        LogLevel::Info
    };

    let entry = LogEntry {
        timestamp: chrono::Local::now(),
        level,
        source: LogSource::Tor,
        message: line.to_string(),
    };
    state.push_log(entry);
}

use anyhow::{bail, Context, Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::core::connection::ConnectionStatus;
use crate::core::state::{LogEntry, LogLevel, LogSource, SharedState, TrafficStats};
use crate::tor::control::{parse_bootstrap_event, parse_bw_event, TorControl};
use crate::tor::process::TorProcess;
use crate::tor::torrc;

pub struct TorBootstrap {
    state: SharedState,
    process: Arc<TorProcess>,
    cancel: Arc<tokio::sync::Notify>,
    control: Arc<Mutex<Option<TorControl>>>,
}

impl TorBootstrap {
    pub fn new(state: SharedState) -> Result<Self> {
        let binary = TorProcess::locate_binary()?;
        Ok(Self {
            state,
            process: Arc::new(TorProcess::new(binary)),
            cancel: Arc::new(tokio::sync::Notify::new()),
            control: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn connect(&self) -> Result<()> {
        self.state
            .set_status(ConnectionStatus::Connecting { progress: 0, stage: "Preparing".into() });

        let settings = self.state.settings_snapshot();

        let torrc_path = torrc::write_torrc(&settings).context("Generating torrc")?;
        self.log(LogLevel::Info, format!("torrc written at {}", torrc_path.display()));

        self.process
            .start(&torrc_path, self.state.clone())
            .await
            .context("Starting tor.exe")?;

        let cookie_path = torrc::tor_data_dir()?.join("control_auth_cookie");
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut control = loop {
            if Instant::now() > deadline {
                self.process.stop().await.ok();
                bail!("Timed out waiting for Tor control port to come up");
            }
            if cookie_path.exists() {
                match TorControl::connect(settings.control_port, &cookie_path).await {
                    Ok(c) => break c,
                    Err(e) => {
                        log::debug!("control connect attempt failed: {e}");
                        sleep(Duration::from_millis(500)).await;
                    }
                }
            } else {
                sleep(Duration::from_millis(250)).await;
            }
        };

        control.subscribe_events().await?;
        self.log(LogLevel::Info, "Subscribed to Tor BW + STATUS_CLIENT events".into());

        {
            let mut guard = self.control.lock().await;
            *guard = Some(control);
        }

        self.spawn_event_pump();

        self.spawn_routing_applier();

        if settings.kill_switch {
            match crate::tor::process::TorProcess::locate_binary() {
                Ok(tor_path) => {
                    match crate::network::firewall::enable_kill_switch(&tor_path) {
                        Ok(_) => self.log(
                            LogLevel::Notice,
                            "Kill Switch active — non-Tor traffic is blocked".into(),
                        ),
                        Err(e) => self.log(
                            LogLevel::Warn,
                            format!("Kill Switch could not be enabled: {}", e),
                        ),
                    }
                }
                Err(e) => {
                    self.log(
                        LogLevel::Warn,
                        format!("Kill Switch skipped — tor.exe path not available: {}", e),
                    );
                }
            }
        }

        Ok(())
    }

    pub async fn new_identity(&self) -> Result<()> {
        let mut guard = self.control.lock().await;
        if let Some(ctrl) = guard.as_mut() {
            ctrl.new_identity().await?;
            self.log(LogLevel::Notice, "New Tor identity requested — circuits rebuilding".into());
        }
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        self.state.set_status(ConnectionStatus::Disconnecting);
        self.cancel.notify_waiters();
        if let Err(e) = crate::network::firewall::disable_kill_switch() {
            log::warn!("Disabling Kill Switch failed: {}", e);
        }

        {
            let mut guard = self.control.lock().await;
            if let Some(ctrl) = guard.as_mut() {
                let _ = ctrl.shutdown().await;
            }
            *guard = None;
        }

        self.process.stop().await.ok();

        let settings = self.state.settings_snapshot();
        if matches!(settings.routing_mode, crate::core::routing::RoutingMode::Proxy) {
            if let Err(e) = crate::network::proxy::disable_system_proxy() {
                log::warn!("Disabling system proxy failed: {}", e);
            }
        }

        self.state.set_status(ConnectionStatus::Disconnected);
        self.log(LogLevel::Info, "Disconnected".into());
        Ok(())
    }

    fn spawn_event_pump(&self) {
        let state = self.state.clone();
        let control = self.control.clone();
        let cancel = self.cancel.clone();

        tokio::spawn(async move {
            let mut last_read: u64 = 0;
            let mut last_written: u64 = 0;
            loop {
                tokio::select! {
                    _ = cancel.notified() => {
                        log::debug!("Event pump cancelled");
                        break;
                    }
                    line_res = async {
                        let mut guard = control.lock().await;
                        if let Some(ctrl) = guard.as_mut() {
                            ctrl.read_event().await
                        } else {
                            Ok(None)
                        }
                    } => {
                        let Ok(Some(line)) = line_res else { break };
                        if let Some((r, w)) = parse_bw_event(&line) {
                            let stats = TrafficStats {
                                bytes_read: last_read.saturating_add(r),
                                bytes_written: last_written.saturating_add(w),
                                down_bps: r,
                                up_bps: w,
                            };
                            last_read = stats.bytes_read;
                            last_written = stats.bytes_written;
                            *state.traffic.write() = stats;
                        }
                        if let Some((progress, stage)) = parse_bootstrap_event(&line) {
                            if progress >= 100 {
                                state.set_status(ConnectionStatus::Connected);
                            } else {
                                state.set_status(ConnectionStatus::Connecting {
                                    progress,
                                    stage,
                                });
                            }
                        }
                    }
                }
            }
        });
    }

    fn spawn_routing_applier(&self) {
        let state = self.state.clone();
        let cancel = self.cancel.clone();
        tokio::spawn(async move {
            let deadline = Instant::now() + Duration::from_secs(180);
            loop {
                tokio::select! {
                    biased;
                    _ = cancel.notified() => return,
                    _ = sleep(Duration::from_millis(500)) => {}
                }
                if state.current_status().is_connected() {
                    let settings = state.settings_snapshot();
                    match settings.routing_mode {
                        crate::core::routing::RoutingMode::Proxy => {
                            if let Err(e) =
                                crate::network::proxy::enable_system_proxy(settings.socks_port)
                            {
                                log::error!("Enabling proxy failed: {}", e);
                            } else {
                                push_app_log(
                                    &state,
                                    LogLevel::Info,
                                    format!(
                                        "System SOCKS proxy enabled on 127.0.0.1:{}",
                                        settings.socks_port
                                    ),
                                );
                            }
                        }
                    }
                    return;
                }
                if Instant::now() > deadline {
                    log::warn!("Routing applier giving up — Tor did not connect in time");
                    return;
                }
            }
        });
    }

    fn log(&self, level: LogLevel, msg: String) {
        push_app_log(&self.state, level, msg);
    }
}

fn push_app_log(state: &SharedState, level: LogLevel, msg: String) {
    state.push_log(LogEntry {
        timestamp: chrono::Local::now(),
        level,
        source: LogSource::App,
        message: msg,
    });
}

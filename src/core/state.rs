use parking_lot::RwLock;
use std::sync::Arc;

use crate::config::settings::Settings;
use crate::core::connection::ConnectionStatus;

#[derive(Debug, Clone, Default)]
pub struct TrafficStats {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub down_bps: u64,
    pub up_bps: u64,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warn,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSource {
    App,
    Tor,
    Dns,
}

pub struct AppState {
    pub status: RwLock<ConnectionStatus>,
    pub settings: RwLock<Settings>,
    pub current_ip: RwLock<Option<String>>,
    pub current_ip_country: RwLock<Option<String>>,
    pub traffic: RwLock<TrafficStats>,
    pub logs: RwLock<Vec<LogEntry>>,
    pub update_log: RwLock<Vec<LogEntry>>,
    pub update_in_progress: std::sync::atomic::AtomicBool,
    pub update_last_result: std::sync::atomic::AtomicU8,
}

const MAX_LOG_ENTRIES: usize = 5000;

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            status: RwLock::new(ConnectionStatus::default()),
            settings: RwLock::new(settings),
            current_ip: RwLock::new(None),
            current_ip_country: RwLock::new(None),
            traffic: RwLock::new(TrafficStats::default()),
            logs: RwLock::new(Vec::with_capacity(512)),
            update_log: RwLock::new(Vec::with_capacity(128)),
            update_in_progress: std::sync::atomic::AtomicBool::new(false),
            update_last_result: std::sync::atomic::AtomicU8::new(0),
        }
    }

    pub fn push_update_log(&self, entry: LogEntry) {
        let mut logs = self.update_log.write();
        logs.push(entry);
        if logs.len() > 1000 {
            let overflow = logs.len() - 1000;
            logs.drain(0..overflow);
        }
    }

    pub fn push_log(&self, entry: LogEntry) {
        let mut logs = self.logs.write();
        logs.push(entry);
        if logs.len() > MAX_LOG_ENTRIES {
            let overflow = logs.len() - MAX_LOG_ENTRIES;
            logs.drain(0..overflow);
        }
    }

    pub fn set_status(&self, status: ConnectionStatus) {
        *self.status.write() = status;
    }

    pub fn current_status(&self) -> ConnectionStatus {
        self.status.read().clone()
    }

    pub fn settings_snapshot(&self) -> Settings {
        self.settings.read().clone()
    }

    pub fn update_settings<F: FnOnce(&mut Settings)>(&self, f: F) {
        let mut settings = self.settings.write();
        f(&mut settings);
        if let Err(e) = settings.save() {
            log::error!("Failed to persist settings: {}", e);
        }
    }
}

pub type SharedState = Arc<AppState>;

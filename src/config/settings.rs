use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::core::routing::{BridgeType, RoutingMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub routing_mode: RoutingMode,
    pub smart_connect: bool,
    pub exit_country: Option<String>,
    pub entry_country: Option<String>,
    pub excluded_countries: Vec<String>,
    pub bridge: BridgeSettings,
    pub kill_switch: bool,
    pub start_with_system: bool,
    pub start_minimized: bool,
    pub minimize_to_tray: bool,
    pub socks_port: u16,
    pub control_port: u16,
    pub dns_port: u16,
    pub theme: String,
    #[serde(default)]
    pub language: crate::config::i18n::Language,
    #[serde(default)]
    pub ultima_actualizacion_tor: Option<String>,
    #[serde(default)]
    pub version_tor_instalada: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeSettings {
    pub enabled: bool,
    pub bridge_type: BridgeType,
    pub custom_bridges: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            routing_mode: RoutingMode::Proxy,
            smart_connect: false,
            exit_country: None,
            entry_country: None,
            excluded_countries: Vec::new(),
            bridge: BridgeSettings::default(),
            kill_switch: false,
            start_with_system: false,
            start_minimized: false,
            minimize_to_tray: false,
            socks_port: 9050,
            control_port: 9051,
            dns_port: 9053,
            theme: "dark".to_string(),
            language: crate::config::i18n::Language::default(),
            ultima_actualizacion_tor: None,
            version_tor_instalada: None,
        }
    }
}

impl Settings {
    pub fn settings_path() -> Result<PathBuf> {
        let proj_dirs = directories::ProjectDirs::from("org", "Onionymous", "Onionymous")
            .context("Could not determine config directory")?;
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)
            .with_context(|| format!("Creating config dir {}", config_dir.display()))?;
        Ok(config_dir.join("settings.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::settings_path()?;
        if !path.exists() {
            log::info!("No settings file found, creating default at {}", path.display());
            let defaults = Self::default();
            defaults.save()?;
            return Ok(defaults);
        }
        let data = fs::read_to_string(&path)
            .with_context(|| format!("Reading settings from {}", path.display()))?;
        let settings: Settings = serde_json::from_str(&data)
            .context("Parsing settings JSON — file may be corrupt")?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::settings_path()?;
        let tmp_path = path.with_extension("json.tmp");
        let data = serde_json::to_string_pretty(self).context("Serializing settings")?;
        fs::write(&tmp_path, data)
            .with_context(|| format!("Writing settings to {}", tmp_path.display()))?;
        fs::rename(&tmp_path, &path)
            .with_context(|| format!("Renaming {} -> {}", tmp_path.display(), path.display()))?;
        Ok(())
    }
}

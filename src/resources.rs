use anyhow::{Context, Result};
use std::path::{Path, PathBuf};


#[cfg(feature = "embedded-tor")]
mod embedded {
    pub const TOR_EXE: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tor/tor.exe"));
    pub const GEOIP: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tor/geoip"));
    pub const GEOIP6: &[u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tor/geoip6"));


    pub const LYREBIRD_EXE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/tor/pluggable_transports/lyrebird.exe"
    ));
    pub const SNOWFLAKE_CLIENT_EXE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/tor/pluggable_transports/snowflake-client.exe"
    ));
    pub const CONJURE_CLIENT_EXE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/tor/pluggable_transports/conjure-client.exe"
    ));
}

pub const LOGO_ICO: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.ico"));

pub fn runtime_dir() -> Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("org", "Onionymous", "Onionymous")
        .context("Could not determine local data directory")?;
    let dir = proj_dirs
        .data_local_dir()
        .join("runtime")
        .join(env!("CARGO_PKG_VERSION"));
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Creating runtime dir {}", dir.display()))?;
    Ok(dir)
}

pub fn ensure_extracted() -> Result<PathBuf> {
    let dir = runtime_dir()?;

    #[cfg(feature = "embedded-tor")]
    {
        extract_if_missing(&dir.join("tor.exe"), embedded::TOR_EXE, true)?;
        extract_if_missing(&dir.join("geoip"), embedded::GEOIP, false)?;
        extract_if_missing(&dir.join("geoip6"), embedded::GEOIP6, false)?;

        let pt_dir = dir.join("pluggable_transports");
        std::fs::create_dir_all(&pt_dir)
            .with_context(|| format!("Creating PT dir {}", pt_dir.display()))?;
        extract_if_missing(&pt_dir.join("lyrebird.exe"), embedded::LYREBIRD_EXE, true)?;
        extract_if_missing(
            &pt_dir.join("snowflake-client.exe"),
            embedded::SNOWFLAKE_CLIENT_EXE,
            true,
        )?;
        extract_if_missing(
            &pt_dir.join("conjure-client.exe"),
            embedded::CONJURE_CLIENT_EXE,
            true,
        )?;
    }

    Ok(dir)
}


fn extract_if_missing(path: &Path, bytes: &[u8], _executable: bool) -> Result<()> {
    if path.exists() {
        if let Ok(meta) = std::fs::metadata(path) {
            if meta.len() == bytes.len() as u64 {
                return Ok(());
            }
        }
    }

    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)
        .with_context(|| format!("Writing embedded resource to {}", tmp.display()))?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("Renaming {} -> {}", tmp.display(), path.display()))?;

    #[cfg(unix)]
    if _executable {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms)?;
    }

    log::info!("Extracted embedded resource to {}", path.display());
    Ok(())
}


#[cfg(feature = "embedded-tor")]
pub fn tor_binary_path() -> Result<PathBuf> {
    Ok(ensure_extracted()?.join(if cfg!(windows) { "tor.exe" } else { "tor" }))
}

#[cfg(not(feature = "embedded-tor"))]
pub fn tor_binary_path() -> Result<PathBuf> {
    anyhow::bail!("Tor binary is not embedded in this build (feature 'embedded-tor' disabled)")
}

pub fn geoip_path() -> Option<PathBuf> {
    let p = runtime_dir().ok()?.join("geoip");
    if p.is_file() { Some(p) } else { None }
}

pub fn geoip6_path() -> Option<PathBuf> {
    let p = runtime_dir().ok()?.join("geoip6");
    if p.is_file() { Some(p) } else { None }
}

pub fn pluggable_transport_path(name: &str) -> Option<PathBuf> {
    let p = runtime_dir().ok()?.join("pluggable_transports").join(name);
    if p.is_file() { Some(p) } else { None }
}

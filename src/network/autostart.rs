use anyhow::{Context, Result};

pub const AUTOSTART_VALUE_NAME: &str = "Onionymous";

pub fn set_autostart(enabled: bool) -> Result<()> {
    #[cfg(windows)]
    {
        if enabled {
            enable_autostart_windows()
        } else {
            disable_autostart_windows()
        }
    }
    #[cfg(not(windows))]
    {
        if enabled {
            log::warn!("Autostart is only implemented on Windows");
        }
        Ok(())
    }
}

pub fn is_autostart_enabled() -> Result<bool> {
    #[cfg(windows)]
    {
        is_autostart_enabled_windows()
    }
    #[cfg(not(windows))]
    {
        Ok(false)
    }
}

#[cfg(windows)]
fn enable_autostart_windows() -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let exe = std::env::current_exe().context("Determining current executable path")?;
    let value = format!("\"{}\" --autostart", exe.display());

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run_key, _) = hkcu
        .create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
        .context("Opening HKCU\\...\\Run registry key")?;
    run_key
        .set_value(AUTOSTART_VALUE_NAME, &value)
        .context("Writing autostart value")?;
    log::info!("Autostart enabled: {}", value);
    Ok(())
}

#[cfg(windows)]
fn disable_autostart_windows() -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu
        .open_subkey_with_flags(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            KEY_SET_VALUE | KEY_QUERY_VALUE,
        )
        .context("Opening HKCU\\...\\Run registry key")?;
    match run_key.delete_value(AUTOSTART_VALUE_NAME) {
        Ok(_) => {
            log::info!("Autostart disabled");
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Deleting autostart value: {}", e)),
    }
}

#[cfg(windows)]
fn is_autostart_enabled_windows() -> Result<bool> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = match hkcu
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
    {
        Ok(k) => k,
        Err(_) => return Ok(false),
    };
    match run_key.get_value::<String, _>(AUTOSTART_VALUE_NAME) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}


pub fn launched_via_autostart() -> bool {
    std::env::args().any(|a| a == "--autostart")
}

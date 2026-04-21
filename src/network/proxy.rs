use anyhow::{Context, Result};

#[cfg(windows)]
mod win {
    use super::*;
    use winreg::enums::*;
    use winreg::RegKey;

    const IE_SETTINGS: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

    pub fn enable(socks_port: u16) -> Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu
            .create_subkey(IE_SETTINGS)
            .context("Opening Internet Settings registry key")?;

        let proxy_string = format!("socks=127.0.0.1:{}", socks_port);
        key.set_value("ProxyEnable", &1u32)
            .context("Setting ProxyEnable")?;
        key.set_value("ProxyServer", &proxy_string)
            .context("Setting ProxyServer")?;
        key.set_value("ProxyOverride", &"localhost;127.0.0.1;<local>")
            .context("Setting ProxyOverride")?;

        broadcast_settings_change();
        Ok(())
    }

    pub fn disable() -> Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu
            .create_subkey(IE_SETTINGS)
            .context("Opening Internet Settings registry key")?;
        key.set_value("ProxyEnable", &0u32)
            .context("Setting ProxyEnable=0")?;
        broadcast_settings_change();
        Ok(())
    }

    fn broadcast_settings_change() {
        use windows::Win32::Networking::WinInet::{
            InternetSetOptionW, INTERNET_OPTION_PROXY_SETTINGS_CHANGED,
            INTERNET_OPTION_REFRESH,
        };
        unsafe {
            let _ =
                InternetSetOptionW(None, INTERNET_OPTION_PROXY_SETTINGS_CHANGED, None, 0);
            let _ = InternetSetOptionW(None, INTERNET_OPTION_REFRESH, None, 0);
        }
    }
}

#[cfg(windows)]
pub fn enable_system_proxy(socks_port: u16) -> Result<()> {
    win::enable(socks_port)
}

#[cfg(windows)]
pub fn disable_system_proxy() -> Result<()> {
    win::disable()
}


#[cfg(not(windows))]
pub fn enable_system_proxy(_socks_port: u16) -> Result<()> {
    log::warn!("System-wide proxy configuration not implemented for this OS");
    Ok(())
}

#[cfg(not(windows))]
pub fn disable_system_proxy() -> Result<()> {
    Ok(())
}

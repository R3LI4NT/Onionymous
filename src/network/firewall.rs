use anyhow::{Context, Result};
use std::process::Command;

const RULE_BLOCK_ALL: &str = "Onionymous_KS_BlockAll";
const RULE_ALLOW_TOR: &str = "Onionymous_KS_AllowTor";

#[cfg(windows)]
pub fn enable_kill_switch(tor_binary: &std::path::Path) -> Result<()> {
    if !is_elevated() {
        anyhow::bail!(
            "Kill Switch requires administrator privileges. \
             Relaunch Onionymous as administrator to use it."
        );
    }

    let _ = disable_kill_switch_quiet();

    netsh(&[
        "advfirewall",
        "firewall",
        "add",
        "rule",
        &format!("name={}", RULE_BLOCK_ALL),
        "dir=out",
        "action=block",
        "enable=yes",
        "profile=any",
    ])
    .context("Adding outbound block-all rule")?;

    netsh(&[
        "advfirewall",
        "firewall",
        "add",
        "rule",
        &format!("name={}", RULE_ALLOW_TOR),
        "dir=out",
        "action=allow",
        &format!("program={}", tor_binary.display()),
        "enable=yes",
        "profile=any",
    ])
    .context("Adding Tor allow rule")?;

    log::info!(
        "Kill Switch enabled. Allowed binary: {}",
        tor_binary.display()
    );
    Ok(())
}

#[cfg(windows)]
pub fn disable_kill_switch() -> Result<()> {
    if !is_elevated() {
        return Ok(());
    }
    disable_kill_switch_quiet();
    log::info!("Kill Switch disabled");
    Ok(())
}

#[cfg(windows)]
fn disable_kill_switch_quiet() {
    let _ = netsh(&[
        "advfirewall",
        "firewall",
        "delete",
        "rule",
        &format!("name={}", RULE_BLOCK_ALL),
    ]);
    let _ = netsh(&[
        "advfirewall",
        "firewall",
        "delete",
        "rule",
        &format!("name={}", RULE_ALLOW_TOR),
    ]);
}

#[cfg(windows)]
pub fn is_elevated() -> bool {
    use std::mem;
    use std::os::windows::io::AsRawHandle;

    let out = Command::new("net")
        .args(["session"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    let _ = mem::size_of::<std::fs::File>();
    let _ = std::io::stdout().as_raw_handle();

    match out {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[cfg(windows)]
trait CreationFlagsExt {
    fn creation_flags(&mut self, flags: u32) -> &mut Self;
}

#[cfg(windows)]
impl CreationFlagsExt for Command {
    fn creation_flags(&mut self, flags: u32) -> &mut Self {
        use std::os::windows::process::CommandExt;
        CommandExt::creation_flags(self, flags)
    }
}

#[cfg(windows)]
fn netsh(args: &[&str]) -> Result<()> {
    let status = Command::new("netsh")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .context("Spawning netsh")?;
    if !status.success() {
        anyhow::bail!("netsh exited with {}", status);
    }
    Ok(())
}


#[cfg(not(windows))]
pub fn enable_kill_switch(_tor_binary: &std::path::Path) -> Result<()> {
    log::warn!("Kill switch is only implemented on Windows");
    Ok(())
}

#[cfg(not(windows))]
pub fn disable_kill_switch() -> Result<()> {
    Ok(())
}

#[cfg(not(windows))]
pub fn is_elevated() -> bool {
    false
}

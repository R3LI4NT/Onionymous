use anyhow::{bail, Context, Result};
use parking_lot::Mutex;
use serde_json::json;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;

pub struct SingBoxProcess {
    child: Option<Child>,
    config_path: PathBuf,
}

impl Drop for SingBoxProcess {
    fn drop(&mut self) {
        if let Some(mut c) = self.child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
        let _ = std::fs::remove_file(&self.config_path);
    }
}

unsafe impl Send for SingBoxProcess {}
unsafe impl Sync for SingBoxProcess {}

static PROC: Mutex<Option<Arc<Mutex<SingBoxProcess>>>> = Mutex::new(None);

pub fn singbox_exe_path() -> Result<PathBuf> {
    let p = crate::resources::runtime_dir()
        .context("Resolviendo runtime dir")?
        .join("sing-box.exe");
    if !p.is_file() {
        bail!(
            "sing-box.exe no encontrado en {}. Ejecutá download-requeriments.bat.",
            p.display()
        );
    }
    Ok(p)
}

pub fn iniciar(socks_port: u16) -> Result<()> {
    let mut guard = PROC.lock();
    if guard.is_some() {
        return Ok(());
    }

    let exe = singbox_exe_path()?;
    let runtime = crate::resources::runtime_dir()?;
    let config_path = runtime.join("singbox-config.json");

    let config = generar_config(socks_port);
    std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)
        .with_context(|| format!("Escribiendo config en {}", config_path.display()))?;

    log::info!(
        "Lanzando sing-box: {} run -D {} -c {}",
        exe.display(),
        runtime.display(),
        config_path.display()
    );

    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let mut cmd = Command::new(&exe);
    cmd.arg("run")
        .arg("-D")
        .arg(&runtime)
        .arg("-c")
        .arg(&config_path)
        .current_dir(&runtime)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW);

    let mut child = cmd.spawn().with_context(|| {
        format!(
            "Spawneando sing-box. Verificá que el archivo {} sea ejecutable y que Onionymous corra como administrador.",
            exe.display()
        )
    })?;

    if let Some(stdout) = child.stdout.take() {
        std::thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                log::info!("[sing-box] {}", line);
            }
        });
    }
    if let Some(stderr) = child.stderr.take() {
        std::thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                log::warn!("[sing-box] {}", line);
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_millis(2500));

    if let Ok(Some(status)) = child.try_wait() {
        bail!(
            "sing-box terminó inmediatamente con código {}. Revisá las líneas [sing-box] en el log.",
            status
        );
    }

    let proc = SingBoxProcess {
        child: Some(child),
        config_path,
    };

    *guard = Some(Arc::new(Mutex::new(proc)));
    log::info!("sing-box activo, TUN -> SOCKS5 :{}", socks_port);
    Ok(())
}

pub fn detener() -> Result<()> {
    let mut guard = PROC.lock();
    if let Some(proc_arc) = guard.take() {
        drop(proc_arc);
        log::info!("sing-box detenido");
    }
    Ok(())
}

pub fn esta_corriendo() -> bool {
    PROC.lock().is_some()
}

fn generar_config(socks_port: u16) -> serde_json::Value {
    json!({
        "log": {
            "level": "info",
            "timestamp": true
        },
        "dns": {
            "servers": [
                {
                    "tag": "remote",
                    "address": "tcp://1.1.1.1",
                    "address_resolver": "local",
                    "detour": "tor-out"
                },
                {
                    "tag": "local",
                    "address": "local",
                    "detour": "direct-out"
                },
                {
                    "tag": "block",
                    "address": "rcode://refused"
                }
            ],
            "rules": [
                {
                    "outbound": "any",
                    "server": "local"
                },
                {
                    "clash_mode": "direct",
                    "server": "local"
                },
                {
                    "clash_mode": "global",
                    "server": "remote"
                }
            ],
            "final": "remote",
            "strategy": "ipv4_only",
            "independent_cache": true,
            "reverse_mapping": true
        },
        "inbounds": [
            {
                "type": "tun",
                "tag": "tun-in",
                "address": ["172.19.0.1/30"],
                "mtu": 1500,
                "auto_route": true,
                "strict_route": true,
                "stack": "system",
                "endpoint_independent_nat": true,
                "platform": {
                    "http_proxy": {
                        "enabled": false
                    }
                }
            }
        ],
        "outbounds": [
            {
                "type": "socks",
                "tag": "tor-out",
                "server": "127.0.0.1",
                "server_port": socks_port,
                "version": "5",
                "udp_over_tcp": true,
                "network": "tcp"
            },
            {
                "type": "direct",
                "tag": "direct-out"
            }
        ],
        "route": {
            "auto_detect_interface": true,
            "rules": [
                {
                    "action": "sniff",
                    "sniffer": ["http", "tls", "quic", "dns"]
                },
                {
                    "protocol": "dns",
                    "action": "hijack-dns"
                },
                {
                    "ip_is_private": true,
                    "outbound": "direct-out"
                },
                {
                    "process_name": [
                        "tor.exe",
                        "sing-box.exe",
                        "lyrebird.exe",
                        "onionymous.exe"
                    ],
                    "outbound": "direct-out"
                },
                {
                    "network": "udp",
                    "outbound": "direct-out"
                }
            ],
            "final": "tor-out"
        }
    })
}

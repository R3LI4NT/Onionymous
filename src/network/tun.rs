use anyhow::{anyhow, bail, Context, Result};
use libloading::{Library, Symbol};
use parking_lot::Mutex;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::sync::Arc;

#[allow(non_camel_case_types)]
type WINTUN_ADAPTER_HANDLE = *mut std::ffi::c_void;
#[allow(non_camel_case_types)]
type WINTUN_SESSION_HANDLE = *mut std::ffi::c_void;
#[allow(non_camel_case_types)]
type DWORD = u32;
#[allow(non_camel_case_types)]
type BOOL = i32;
#[allow(non_camel_case_types)]
type LPCWSTR = *const u16;

type WintunCreateAdapter = unsafe extern "system" fn(
    name: LPCWSTR,
    tunnel_type: LPCWSTR,
    requested_guid: *const GUID,
) -> WINTUN_ADAPTER_HANDLE;

type WintunCloseAdapter = unsafe extern "system" fn(adapter: WINTUN_ADAPTER_HANDLE);

type WintunOpenAdapter =
    unsafe extern "system" fn(name: LPCWSTR) -> WINTUN_ADAPTER_HANDLE;

type WintunStartSession = unsafe extern "system" fn(
    adapter: WINTUN_ADAPTER_HANDLE,
    capacity: DWORD,
) -> WINTUN_SESSION_HANDLE;

type WintunEndSession = unsafe extern "system" fn(session: WINTUN_SESSION_HANDLE);

type WintunGetAdapterLuid = unsafe extern "system" fn(
    adapter: WINTUN_ADAPTER_HANDLE,
    luid: *mut u64,
);

type WintunGetRunningDriverVersion = unsafe extern "system" fn() -> DWORD;

#[repr(C)]
#[derive(Clone, Copy)]
struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

const ADAPTER_NAME: &str = "Onionymous";
const TUNNEL_TYPE: &str = "Onionymous";
const SESSION_CAPACITY: u32 = 0x40_0000;

pub struct TunBackend {
    adapter: WINTUN_ADAPTER_HANDLE,
    session: WINTUN_SESSION_HANDLE,
    close_adapter: WintunCloseAdapter,
    end_session: WintunEndSession,
    luid: u64,
    _lib: Library,
}

unsafe impl Send for TunBackend {}
unsafe impl Sync for TunBackend {}

impl TunBackend {
    pub fn luid(&self) -> u64 {
        self.luid
    }
}

impl Drop for TunBackend {
    fn drop(&mut self) {
        unsafe {
            if !self.session.is_null() {
                (self.end_session)(self.session);
                self.session = null_mut();
            }
            if !self.adapter.is_null() {
                (self.close_adapter)(self.adapter);
                self.adapter = null_mut();
            }
        }
    }
}

static BACKEND: Mutex<Option<Arc<TunBackend>>> = Mutex::new(None);

pub fn wintun_dll_path() -> Result<PathBuf> {
    let runtime = crate::resources::runtime_dir()
        .context("No se pudo resolver el directorio de runtime")?;
    let p = runtime.join("wintun.dll");
    if !p.is_file() {
        bail!(
            "wintun.dll no encontrado en {}. Ejecutá download-requeriments.bat para descargarlo.",
            p.display()
        );
    }
    Ok(p)
}

pub fn habilitar_tun() -> Result<()> {
    if !crate::network::firewall::is_elevated() {
        bail!("El modo TUN requiere privilegios de administrador. Reiniciá Onionymous como administrador.");
    }

    let mut guard = BACKEND.lock();
    if guard.is_some() {
        return Ok(());
    }

    let dll_path = wintun_dll_path()?;
    let lib = unsafe { Library::new(&dll_path) }
        .with_context(|| format!("No se pudo cargar wintun.dll desde {}", dll_path.display()))?;

    let create_adapter_fn: WintunCreateAdapter;
    let close_adapter_fn: WintunCloseAdapter;
    let open_adapter_fn: WintunOpenAdapter;
    let start_session_fn: WintunStartSession;
    let end_session_fn: WintunEndSession;
    let get_luid_fn: WintunGetAdapterLuid;
    let driver_version_fn: WintunGetRunningDriverVersion;

    unsafe {
        let s: Symbol<WintunCreateAdapter> = lib
            .get(b"WintunCreateAdapter\0")
            .context("WintunCreateAdapter no encontrada en wintun.dll")?;
        create_adapter_fn = *s;
        let s: Symbol<WintunCloseAdapter> = lib
            .get(b"WintunCloseAdapter\0")
            .context("WintunCloseAdapter no encontrada en wintun.dll")?;
        close_adapter_fn = *s;
        let s: Symbol<WintunOpenAdapter> = lib
            .get(b"WintunOpenAdapter\0")
            .context("WintunOpenAdapter no encontrada en wintun.dll")?;
        open_adapter_fn = *s;
        let s: Symbol<WintunStartSession> = lib
            .get(b"WintunStartSession\0")
            .context("WintunStartSession no encontrada en wintun.dll")?;
        start_session_fn = *s;
        let s: Symbol<WintunEndSession> = lib
            .get(b"WintunEndSession\0")
            .context("WintunEndSession no encontrada en wintun.dll")?;
        end_session_fn = *s;
        let s: Symbol<WintunGetAdapterLuid> = lib
            .get(b"WintunGetAdapterLUID\0")
            .context("WintunGetAdapterLUID no encontrada en wintun.dll")?;
        get_luid_fn = *s;
        let s: Symbol<WintunGetRunningDriverVersion> = lib
            .get(b"WintunGetRunningDriverVersion\0")
            .context("WintunGetRunningDriverVersion no encontrada en wintun.dll")?;
        driver_version_fn = *s;
    }

    let name_w: Vec<u16> = OsStr::new(ADAPTER_NAME).encode_wide().chain(std::iter::once(0)).collect();
    let type_w: Vec<u16> = OsStr::new(TUNNEL_TYPE).encode_wide().chain(std::iter::once(0)).collect();

    let existing = unsafe { (open_adapter_fn)(name_w.as_ptr()) };
    if !existing.is_null() {
        unsafe { (close_adapter_fn)(existing) };
    }

    let adapter = unsafe { (create_adapter_fn)(name_w.as_ptr(), type_w.as_ptr(), null_mut()) };
    if adapter.is_null() {
        let err = std::io::Error::last_os_error();
        bail!(
            "WintunCreateAdapter falló: {}. Verificá que Onionymous corra como administrador y que wintun.dll sea legítimo.",
            err
        );
    }

    let version = unsafe { (driver_version_fn)() };
    log::info!(
        "Wintun cargado, versión driver: {}.{}",
        version >> 16,
        version & 0xFFFF
    );

    let session = unsafe { (start_session_fn)(adapter, SESSION_CAPACITY) };
    if session.is_null() {
        let err = std::io::Error::last_os_error();
        unsafe { (close_adapter_fn)(adapter) };
        bail!("WintunStartSession falló: {}", err);
    }

    let mut luid: u64 = 0;
    unsafe { (get_luid_fn)(adapter, &mut luid) };

    let backend = Arc::new(TunBackend {
        adapter,
        session,
        close_adapter: close_adapter_fn,
        end_session: end_session_fn,
        luid,
        _lib: lib,
    });

    configurar_interfaz_tun(luid)?;
    aplicar_rutas_tun(luid)?;

    log::info!(
        "Adaptador TUN '{}' levantado correctamente (LUID={})",
        ADAPTER_NAME,
        luid
    );

    *guard = Some(backend);
    Ok(())
}

pub fn deshabilitar_tun() -> Result<()> {
    let mut guard = BACKEND.lock();
    if let Some(backend) = guard.take() {
        let luid = backend.luid();
        drop(backend);
        if let Err(e) = remover_rutas_tun(luid) {
            log::warn!("Removiendo rutas TUN: {}", e);
        }
        log::info!("Adaptador TUN cerrado");
    }
    Ok(())
}

pub fn esta_activo() -> bool {
    BACKEND.lock().is_some()
}

fn configurar_interfaz_tun(luid: u64) -> Result<()> {
    let alias = obtener_alias_por_luid(luid)?;
    run_netsh(&[
        "interface",
        "ipv4",
        "set",
        "address",
        &format!("name={}", alias),
        "static",
        "10.7.0.2",
        "255.255.255.0",
    ])
    .context("Asignando IP estática al adaptador TUN")?;

    run_netsh(&[
        "interface",
        "ipv4",
        "set",
        "interface",
        &alias,
        "metric=1",
    ])
    .ok();

    Ok(())
}

fn aplicar_rutas_tun(luid: u64) -> Result<()> {
    let alias = obtener_alias_por_luid(luid)?;
    run_netsh(&[
        "interface",
        "ipv4",
        "add",
        "route",
        "0.0.0.0/1",
        &alias,
        "0.0.0.0",
        "metric=1",
    ])
    .ok();
    run_netsh(&[
        "interface",
        "ipv4",
        "add",
        "route",
        "128.0.0.0/1",
        &alias,
        "0.0.0.0",
        "metric=1",
    ])
    .ok();
    Ok(())
}

fn remover_rutas_tun(luid: u64) -> Result<()> {
    let alias = obtener_alias_por_luid(luid).unwrap_or_else(|_| ADAPTER_NAME.to_string());
    run_netsh(&[
        "interface",
        "ipv4",
        "delete",
        "route",
        "0.0.0.0/1",
        &alias,
    ])
    .ok();
    run_netsh(&[
        "interface",
        "ipv4",
        "delete",
        "route",
        "128.0.0.0/1",
        &alias,
    ])
    .ok();
    Ok(())
}

fn obtener_alias_por_luid(_luid: u64) -> Result<String> {
    Ok(ADAPTER_NAME.to_string())
}

fn run_netsh(args: &[&str]) -> Result<()> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let status = Command::new("netsh")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .context("Spawneando netsh")?;
    if !status.success() {
        return Err(anyhow!("netsh terminó con {}", status));
    }
    Ok(())
}

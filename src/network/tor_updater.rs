#![allow(clippy::needless_return)]

use anyhow::{anyhow, bail, Context, Result};
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::core::state::{LogEntry, LogLevel, LogSource, SharedState};

const URL_INDICE: &str = "https://archive.torproject.org/tor-package-archive/torbrowser/";
const VERSION_FALLBACK: &str = "14.5.0";
const ARQUITECTURA: &str = "windows-x86_64";

pub struct Reportero {
    estado: SharedState,
}

impl Reportero {
    pub fn nuevo(estado: SharedState) -> Self {
        Self { estado }
    }

    pub fn info(&self, mensaje: impl Into<String>) {
        self.registrar(LogLevel::Info, mensaje.into());
    }

    pub fn aviso(&self, mensaje: impl Into<String>) {
        self.registrar(LogLevel::Warn, mensaje.into());
    }

    pub fn error(&self, mensaje: impl Into<String>) {
        self.registrar(LogLevel::Error, mensaje.into());
    }

    pub fn exito(&self, mensaje: impl Into<String>) {
        self.registrar(LogLevel::Notice, mensaje.into());
    }

    fn registrar(&self, nivel: LogLevel, mensaje: String) {
        let entrada = LogEntry {
            timestamp: chrono::Local::now(),
            level: nivel,
            source: LogSource::App,
            message: mensaje,
        };
        self.estado.push_update_log(entrada);
    }
}

pub async fn consultar_ultima_version(reportero: &Reportero) -> Result<String> {
    reportero.info(format!("Consultando índice de versiones en {}", URL_INDICE));
    let cliente = reqwest::Client::builder()
        .user_agent("Onionymous-Updater/1.0")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let respuesta = cliente.get(URL_INDICE).send().await;
    let cuerpo = match respuesta {
        Ok(r) if r.status().is_success() => r.text().await?,
        Ok(r) => {
            reportero.aviso(format!(
                "El índice respondió con estado {}. Usando versión de respaldo {}.",
                r.status(),
                VERSION_FALLBACK
            ));
            return Ok(VERSION_FALLBACK.to_string());
        }
        Err(e) => {
            reportero.aviso(format!(
                "No se pudo alcanzar el índice ({e}). Usando versión de respaldo {}.",
                VERSION_FALLBACK
            ));
            return Ok(VERSION_FALLBACK.to_string());
        }
    };

    let version = extraer_version_mas_alta(&cuerpo).unwrap_or_else(|| {
        reportero.aviso(format!(
            "No se pudo parsear el índice. Usando versión de respaldo {}.",
            VERSION_FALLBACK
        ));
        VERSION_FALLBACK.to_string()
    });

    reportero.info(format!("Última versión estable detectada: {}", version));
    Ok(version)
}

fn extraer_version_mas_alta(html: &str) -> Option<String> {
    let mut candidatas: Vec<(u32, u32, u32, String)> = Vec::new();
    let mut resto = html;
    loop {
        let Some(inicio) = resto.find("href=\"") else { break };
        resto = &resto[inicio + 6..];
        let Some(fin) = resto.find('"') else { break };
        let enlace = &resto[..fin];
        resto = &resto[fin + 1..];

        let enlace = enlace.trim_end_matches('/');
        let partes: Vec<&str> = enlace.split('.').collect();
        if partes.len() < 2 {
            continue;
        }
        let mayor = partes.first().and_then(|s| s.parse::<u32>().ok());
        let menor = partes.get(1).and_then(|s| s.parse::<u32>().ok());
        let parche = partes.get(2).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        if let (Some(mayor), Some(menor)) = (mayor, menor) {
            candidatas.push((mayor, menor, parche, enlace.to_string()));
        }
    }
    candidatas.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(b.2.cmp(&a.2)));
    candidatas.into_iter().next().map(|c| c.3)
}

pub async fn descargar_e_instalar(
    estado: SharedState,
    reportero: Arc<Reportero>,
) -> Result<()> {
    let version = consultar_ultima_version(&reportero).await?;

    let directorio_temp = std::env::temp_dir().join(format!("onionymous-update-{}", std::process::id()));
    if directorio_temp.exists() {
        let _ = std::fs::remove_dir_all(&directorio_temp);
    }
    std::fs::create_dir_all(&directorio_temp)?;

    let nombre_archivo = format!("tor-expert-bundle-{}-{}.tar.gz", ARQUITECTURA, version);
    let url_archivo = format!("{}{}/{}", URL_INDICE, version, nombre_archivo);
    let url_checksum = format!("{}{}/sha256sums-signed-build.txt", URL_INDICE, version);

    let ruta_archivo = directorio_temp.join(&nombre_archivo);

    descargar_con_progreso(&url_archivo, &ruta_archivo, &reportero).await
        .with_context(|| format!("Descarga del paquete desde {}", url_archivo))?;

    reportero.info("Obteniendo checksum oficial...");
    let checksum_esperado = obtener_checksum_esperado(&url_checksum, &nombre_archivo, &reportero).await?;

    reportero.info("Verificando integridad del archivo...");
    let checksum_calculado = calcular_sha256(&ruta_archivo)?;
    if !checksum_calculado.eq_ignore_ascii_case(&checksum_esperado) {
        reportero.error(format!(
            "Checksum no coincide. Esperado: {}. Calculado: {}.",
            checksum_esperado, checksum_calculado
        ));
        let _ = std::fs::remove_dir_all(&directorio_temp);
        bail!("Integridad del paquete descargado comprometida. Operación abortada.");
    }
    reportero.exito("Integridad verificada correctamente.");

    reportero.info("Descomprimiendo archivo...");
    let directorio_extraccion = directorio_temp.join("extraido");
    std::fs::create_dir_all(&directorio_extraccion)?;
    extraer_tar_gz(&ruta_archivo, &directorio_extraccion)?;

    reportero.info("Instalando nuevos binarios...");
    instalar_en_runtime(&directorio_extraccion, &reportero)?;

    let _ = std::fs::remove_dir_all(&directorio_temp);

    reportero.exito(format!(
        "Tor actualizado exitosamente a la versión {}. Los próximos arranques usarán los nuevos binarios.",
        version
    ));

    let ahora = chrono::Local::now().to_rfc3339();
    let mut settings = estado.settings.write();
    settings.ultima_actualizacion_tor = Some(ahora);
    settings.version_tor_instalada = Some(version.clone());
    let _ = settings.save();

    Ok(())
}

async fn descargar_con_progreso(
    url: &str,
    destino: &Path,
    reportero: &Reportero,
) -> Result<()> {
    reportero.info(format!("Descargando {}", url));
    let cliente = reqwest::Client::builder()
        .user_agent("Onionymous-Updater/1.0")
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    let respuesta = cliente.get(url).send().await?;
    if !respuesta.status().is_success() {
        bail!("Descarga falló con estado HTTP {}", respuesta.status());
    }

    let tamano_total = respuesta.content_length();
    if let Some(tam) = tamano_total {
        reportero.info(format!("Tamaño del paquete: {}", formato_bytes(tam)));
    }

    let mut archivo = tokio::fs::File::create(destino).await?;
    let mut bytes_recibidos: u64 = 0;
    let mut ultimo_reporte: u64 = 0;

    let mut flujo = respuesta.bytes_stream();
    while let Some(fragmento) = flujo.next().await {
        let fragmento = fragmento?;
        archivo.write_all(&fragmento).await?;
        bytes_recibidos += fragmento.len() as u64;

        if bytes_recibidos - ultimo_reporte > 512 * 1024 {
            ultimo_reporte = bytes_recibidos;
            match tamano_total {
                Some(total) if total > 0 => {
                    let porcentaje = (bytes_recibidos * 100) / total;
                    reportero.info(format!(
                        "Progreso: {}% ({} / {})",
                        porcentaje,
                        formato_bytes(bytes_recibidos),
                        formato_bytes(total)
                    ));
                }
                _ => {
                    reportero.info(format!("Recibido: {}", formato_bytes(bytes_recibidos)));
                }
            }
        }
    }

    archivo.flush().await?;
    reportero.info(format!("Descarga completa ({})", formato_bytes(bytes_recibidos)));
    Ok(())
}

async fn obtener_checksum_esperado(
    url: &str,
    nombre_archivo: &str,
    reportero: &Reportero,
) -> Result<String> {
    let cliente = reqwest::Client::builder()
        .user_agent("Onionymous-Updater/1.0")
        .timeout(std::time::Duration::from_secs(20))
        .build()?;

    let texto = cliente.get(url).send().await?.text().await?;

    for linea in texto.lines() {
        let partes: Vec<&str> = linea.split_whitespace().collect();
        if partes.len() >= 2 {
            let archivo_registrado = partes[1].trim_start_matches('*');
            if archivo_registrado == nombre_archivo {
                return Ok(partes[0].to_string());
            }
        }
    }

    reportero.aviso(format!(
        "Archivo de checksums no contiene entrada para {}; revisar formato del publicador.",
        nombre_archivo
    ));
    Err(anyhow!("No se encontró checksum para {}", nombre_archivo))
}

fn calcular_sha256(ruta: &Path) -> Result<String> {
    let mut archivo = std::fs::File::open(ruta)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let leidos = archivo.read(&mut buffer)?;
        if leidos == 0 {
            break;
        }
        hasher.update(&buffer[..leidos]);
    }
    let digest = hasher.finalize();
    Ok(hex_encode(&digest))
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn extraer_tar_gz(origen: &Path, destino: &Path) -> Result<()> {
    let archivo = std::fs::File::open(origen)?;
    let descomprimido = GzDecoder::new(archivo);
    let mut tar = tar::Archive::new(descomprimido);
    tar.unpack(destino)?;
    Ok(())
}

fn instalar_en_runtime(directorio_extraido: &Path, reportero: &Reportero) -> Result<()> {
    let destino = crate::resources::runtime_dir()
        .context("Obteniendo directorio de runtime")?;

    let directorio_tor = directorio_extraido.join("tor");
    if !directorio_tor.is_dir() {
        bail!("El archivo extraído no contiene la carpeta esperada 'tor/'");
    }

    copiar_recursivo(&directorio_tor, &destino, reportero)?;

    let directorio_data = directorio_extraido.join("data");
    if directorio_data.is_dir() {
        for archivo in ["geoip", "geoip6"] {
            let origen = directorio_data.join(archivo);
            if origen.is_file() {
                let destino_archivo = destino.join(archivo);
                std::fs::copy(&origen, &destino_archivo)?;
                reportero.info(format!("Instalado {}", archivo));
            }
        }
    }

    Ok(())
}

fn copiar_recursivo(origen: &Path, destino: &Path, reportero: &Reportero) -> Result<()> {
    std::fs::create_dir_all(destino)?;
    for entrada in std::fs::read_dir(origen)? {
        let entrada = entrada?;
        let tipo = entrada.file_type()?;
        let ruta_origen = entrada.path();
        let ruta_destino = destino.join(entrada.file_name());

        if tipo.is_dir() {
            copiar_recursivo(&ruta_origen, &ruta_destino, reportero)?;
        } else if tipo.is_file() {
            let nombre_temporal = ruta_destino.with_extension("nuevo");
            std::fs::copy(&ruta_origen, &nombre_temporal)?;
            if ruta_destino.exists() {
                std::fs::remove_file(&ruta_destino)?;
            }
            std::fs::rename(&nombre_temporal, &ruta_destino)?;
            if let Some(nombre) = entrada.file_name().to_str() {
                reportero.info(format!("Instalado {}", nombre));
            }
        }
    }
    Ok(())
}

fn formato_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn detectar_version_instalada() -> Option<String> {
    use std::process::Command;

    let tor_path = crate::resources::tor_binary_path().ok()?;
    let mut cmd = Command::new(&tor_path);
    cmd.arg("--version");

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let output = cmd.output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout
        .lines()
        .next()?
        .split_whitespace()
        .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))?
        .trim_end_matches('.')
        .to_string();
    Some(version)
}
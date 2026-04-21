<p align="center">
  <img width="265" height="260" alt="Onionymous" src="https://github.com/user-attachments/assets/6cab7c7d-9f99-4381-8671-832d3772747a" />
</p>


<p align="center">
  <em>A modern, privacy-first desktop client for the Tor network.</em><br/>
  <em>Un cliente de escritorio moderno para la red Tor, enfocado en privacidad.</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-8a5cff?style=flat-square" alt="version" />
  <img src="https://img.shields.io/badge/platform-Windows%2010%2B-0078d4?style=flat-square" alt="platform" />
  <img src="https://img.shields.io/badge/built%20with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white" alt="rust" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="license" /></a>
  <img src="https://img.shields.io/badge/portable-yes-8a5cff?style=flat-square" alt="portable" />
</p>

<p align="center">
  <a href="#english">English</a> · <a href="#español">Español</a>
</p>

---

<a id="english"></a>

## English

### Overview

**Onionymous** is a native Windows desktop application that routes your traffic through the Tor network with a modern, opinionated UI. The official Tor binary, geoip databases, and pluggable transports are all embedded inside a single portable `.exe` — no installer, no external folders, no hand-edited config files.

It is aimed at users who want the privacy and censorship-circumvention benefits of Tor without the complexity of running Tor manually or configuring a SOCKS proxy by hand. Onionymous manages the Tor process for you, watches the bootstrap in real time, rotates identities on demand, and shows connection state and bandwidth at a glance.

### ✨ Features (v1.0.0)

#### 🎯 Connection control
- **Click-the-orb UX** — the central animated circle *is* the connect/disconnect button. No extra buttons cluttering the dashboard
- **Live bootstrap progress** painted as a filled arc around the orb, tracking the Tor bootstrap percentage as it happens
- **Visual feedback for every state** — power icon when off, spinner while connecting, shield-check when connected, warning on failure
- **Pulsating glow** when connected, directional chirp sounds when toggling state
- **Public IP display in amber** on the dashboard, updated on every connect and after each identity rotation
- **Automatic cleanup** of stale `tor.exe` processes that would otherwise block the SOCKS port on restart

#### 🌐 Routing & privacy
- **Local SOCKS5 proxy** on `127.0.0.1:9050` — Tor-aware apps get routed automatically via system proxy settings
- **Exit country selector** — choose from 40+ countries for your exit relay, or leave it on *Automatic*
- **Excluded countries list** — permanently blacklist countries from being used as guard/middle/exit nodes
- **Bridges / pluggable transports** for censored networks:
  - **obfs4** — most common, fast, good for light censorship
  - **Snowflake** — WebRTC-based, good for heavy censorship
  - **Custom bridges** — paste your own lines from bridges.torproject.org
- **New Identity** button — forces Tor to rebuild circuits and present a fresh IP
- **Refresh IP** button — re-queries your public IP without rebuilding circuits

#### 🛡️ Security
- **Kill Switch** via Windows Defender Firewall — blocks all non-Tor traffic at the OS level (requires admin). Automatically torn down on disconnect so you're never left without internet
- **Automatic admin detection** — the switch is offered but gracefully skipped if you're running as a normal user, with a clear log message instead of failing silently
- **Cookie-authenticated control port** — no passwords on disk, Tor rotates the cookie on every launch

#### 📊 Observability
- **Real-time bandwidth sparkline** with violet download / amber upload, 60-sample rolling window, normalised to local peak
- **Session totals** for bytes read and written
- **Full event log** combining Tor and app events, filterable by source and level, with auto-scroll to the tail
- **Circuit summary card** showing exit country and bridge status at a glance

#### 🔧 Tools
- **Update Tor** button — downloads the latest Tor Expert Bundle directly from the official Tor Project archive at `archive.torproject.org`, verifies its SHA-256 checksum against the signed `sha256sums-signed-build.txt` manifest, and installs the new binaries atomically
- **Live terminal output** with timestamped, severity-colored lines while the update runs
- **Installed version detection** with timestamp of last successful update, cached per session so it doesn't hammer `tor.exe` on every frame

#### 🎨 User experience
- **Custom violet theme** with DWM Mica / Acrylic blur on Windows 10 & 11
- **Animated circular status orb** as the focal UI element — painted from scratch with the egui painter API
- **Phosphor Icons** throughout for consistent, sharp iconography that renders on any system regardless of installed fonts
- **Synthesized UI sounds** — click, tick, connect chirp, disconnect chirp, error warble — all generated in memory by `rodio`, no audio files bundled
- **Bilingual** — English and Spanish, switchable at runtime via the globe button in the header
- **True portability** — the `.exe` extracts its embedded runtime to `%LocalAppData%\Onionymous\runtime\<version>\` on first launch, then runs from that cache. Move the `.exe` anywhere and it still works

#### 🖥️ System integration
- **Start with Windows** via `HKCU\...\Run` (idempotent registry entry)
- **Start minimized** when launched by autostart
- **Optional minimize-to-tray** on close (off by default to avoid confusing first-time users)
- **Clean shutdown** button — disconnects Tor, removes the proxy, tears down the kill switch, then exits

### ⚠️ What Onionymous is *not*

Being upfront about the limits:

- **It is not the Tor Browser.** Onionymous routes your traffic, but it doesn't give you browser-level anti-fingerprinting protections. For web browsing where strong anonymity actually matters, use the Tor Browser
- **It is not a system-wide VPN.** Apps that ignore the system proxy won't be routed through Tor. Only proxy-aware apps (most modern browsers, many desktop clients) will pass through
- **It is Windows-only for now.** The core logic is cross-platform Rust, but the packaging, firewall integration, autostart, and DWM blur are Windows-specific

### 📦 Installation

Download `onionymous.exe` from the [Releases](https://github.com/R3LI4NT/Onionymous/releases) page and run it.

That's it. The `.exe` is fully self-contained — no installer, no runtime dependencies.

### 🔨 Building from source

Requirements:
- [Rust 1.78+](https://rustup.rs)
- `tar.exe` and `curl.exe` (both ship with Windows 10 1803+)

```powershell
# 1. Fetch the Tor Expert Bundle once. This downloads tor.exe, geoip
#    databases and the pluggable transports into ./resources/tor/.
#    MUST run BEFORE cargo build — the binaries are embedded into the
#    .exe at compile time.
.\download-requeriments.bat

# 2. Build the portable release executable.
cargo build --release
```

The resulting `target\release\onionymous.exe` is around **90 MB** (Tor + pluggable transports are embedded) and fully portable. Move it wherever you like.

For faster iteration during development:
```powershell
cargo build --no-default-features
Copy-Item -Recurse -Force .\resources .\target\debug\resources
```

### 🤝 Credits

- Developed by [**@R3LI4NT**](https://github.com/R3LI4NT)
- Built on top of the [Tor Project](https://www.torproject.org/) — this project is not affiliated with or endorsed by the Tor Project
- Uses [egui](https://github.com/emilk/egui) for the GUI, [Phosphor Icons](https://phosphoricons.com/) for iconography, [rodio](https://github.com/RustAudio/rodio) for synthesized audio

### 📜 License

MIT. See [LICENSE](LICENSE).

---

<a id="español"></a>

## Español

### Resumen

**Onionymous** es una aplicación nativa de escritorio para Windows que enruta tu tráfico a través de la red Tor con una UI moderna y cuidada. El binario oficial de Tor, las bases de datos geoip y los pluggable transports vienen todos embebidos dentro de un único `.exe` portable — sin instalador, sin carpetas aparte, sin archivos de configuración para editar a mano.

Está pensado para quienes quieren los beneficios de privacidad y anti-censura de Tor sin la complejidad de correr Tor manualmente o configurar un proxy SOCKS a mano. Onionymous maneja el proceso de Tor por vos, observa el bootstrap en tiempo real, rota identidades cuando lo pedís, y te muestra el estado de conexión y ancho de banda de un vistazo.

### ✨ Características (v1.0.0)

#### 🎯 Control de conexión
- **UX click-en-el-orb** — el círculo animado central *es* el botón de conectar/desconectar. Sin botones extra que ensucien el dashboard
- **Progreso de bootstrap en vivo** pintado como un arco que se llena alrededor del orb, siguiendo el porcentaje de bootstrap de Tor en tiempo real
- **Feedback visual para cada estado** — icono de power cuando está apagado, spinner mientras conecta, shield-check cuando está conectado, warning en caso de falla
- **Glow pulsante** cuando está conectado, sonidos direccionales al cambiar de estado
- **IP pública en ámbar** en el dashboard, actualizada en cada conexión y después de cada rotación de identidad
- **Limpieza automática** de procesos `tor.exe` zombi que ocuparían el puerto SOCKS al reiniciar

#### 🌐 Enrutamiento y privacidad
- **Proxy SOCKS5 local** en `127.0.0.1:9050` — las apps compatibles con Tor se enrutan automáticamente vía el proxy del sistema
- **Selector de país de salida** — elegí entre 40+ países para tu relay de salida, o dejalo en *Automático*
- **Lista de países excluidos** — vetá permanentemente países para que nunca se usen como guard/middle/exit
- **Puentes / pluggable transports** para redes censuradas:
  - **obfs4** — el más común, rápido, bueno para censura leve
  - **Snowflake** — basado en WebRTC, bueno para censura fuerte
  - **Puentes personalizados** — pegá tus propias líneas desde bridges.torproject.org
- **Botón Nueva Identidad** — fuerza a Tor a reconstruir circuitos y presentar una IP nueva
- **Botón Actualizar IP** — vuelve a consultar tu IP pública sin reconstruir circuitos

#### 🛡️ Seguridad
- **Kill Switch** vía Windows Defender Firewall — bloquea todo el tráfico no-Tor a nivel OS (requiere admin). Se desactiva automáticamente al desconectar para que nunca te quedes sin internet
- **Detección automática de admin** — el switch se ofrece pero se omite limpiamente si corrés como usuario normal, con un mensaje claro en el log en lugar de fallar silenciosamente
- **Control port con cookie authentication** — cero contraseñas en disco, Tor rota la cookie en cada arranque

#### 📊 Observabilidad
- **Sparkline de ancho de banda en tiempo real** con descarga en violeta y subida en ámbar, ventana rolante de 60 muestras, normalizada al pico local
- **Totales de sesión** para bytes leídos y escritos
- **Registro completo de eventos** combinando Tor + app, filtrable por origen y nivel, con auto-scroll al pie
- **Card de resumen del circuito** mostrando país de salida y estado de puentes de un vistazo

#### 🔧 Herramientas
- **Botón Actualizar Tor** — descarga el Tor Expert Bundle más reciente directamente del archivo oficial del Tor Project en `archive.torproject.org`, verifica el checksum SHA-256 contra el manifiesto firmado `sha256sums-signed-build.txt`, e instala los binarios nuevos de forma atómica
- **Terminal en vivo** con líneas timestamped y coloreadas por nivel de severidad mientras corre la actualización
- **Detección de versión instalada** con timestamp de la última actualización exitosa, cacheada por sesión para no golpear `tor.exe` en cada frame

#### 🎨 Experiencia de usuario
- **Tema violeta propio** con blur DWM Mica / Acrylic en Windows 10 y 11
- **Orb circular animado** como elemento focal de la UI — pintado desde cero con la API painter de egui
- **Phosphor Icons** en toda la app para iconografía consistente y nítida que renderiza en cualquier sistema independientemente de las fuentes instaladas
- **Sonidos UI sintetizados** — click, tick, chirp de conexión, chirp de desconexión, warble de error — todos generados en memoria por `rodio`, cero archivos de audio empaquetados
- **Bilingüe** — inglés y español, intercambiables en tiempo real con el botón del globo en el header
- **Portabilidad real** — el `.exe` extrae su runtime embebido a `%LocalAppData%\Onionymous\runtime\<versión>\` en el primer arranque, y después corre desde ese cache. Movelo donde quieras y sigue funcionando

#### 🖥️ Integración con el sistema
- **Iniciar con Windows** vía `HKCU\...\Run` (entrada de registro idempotente)
- **Iniciar minimizado** cuando arranca por autostart
- **Minimizar a la bandeja al cerrar** (opcional, desactivado por defecto para no confundir al usuario nuevo)
- **Botón Salir** — desconecta Tor, saca el proxy, desmonta el kill switch, y cierra la app de forma limpia

### ⚠️ Qué *no* es Onionymous

Siendo claros con los límites:

- **No es el Tor Browser.** Onionymous enruta tu tráfico, pero no te da las protecciones anti-fingerprinting a nivel navegador. Para navegación web donde el anonimato realmente importa, usá el Tor Browser
- **No es una VPN para todo el sistema.** Las apps que ignoren el proxy del sistema no van a pasar por Tor. Solo las compatibles con proxy (la mayoría de los navegadores modernos, muchos clientes de escritorio) se van a enrutar
- **Solo Windows por ahora.** La lógica core es Rust multiplataforma, pero el empaquetado, integración con firewall, autostart y blur DWM son específicos de Windows

### 📦 Instalación

Descargá `onionymous.exe` de la página de [Releases](https://github.com/R3LI4NT/Onionymous/releases) y ejecutalo.

Y listo. El `.exe` es totalmente autónomo — sin instalador, sin dependencias de runtime.

### 🔨 Compilar desde el código fuente

Requisitos:
- [Rust 1.78+](https://rustup.rs)
- `tar.exe` y `curl.exe` (ambos vienen con Windows 10 1803+)

```powershell
# 1. Descargar el Tor Expert Bundle una sola vez. Esto baja tor.exe,
#    las bases de datos geoip y los pluggable transports a ./resources/tor/.
#    SE DEBE ejecutar ANTES de cargo build — los binarios se embeben en
#    el .exe en tiempo de compilación.
.\download-requeriments.bat

# 2. Build del ejecutable portable en release.
cargo build --release
```

El `target\release\onionymous.exe` resultante pesa alrededor de **90 MB** (Tor + pluggable transports embebidos) y es totalmente portable. Movelo a donde quieras.

Para iteración más rápida durante desarrollo:
```powershell
cargo build --no-default-features
Copy-Item -Recurse -Force .\resources .\target\debug\resources
```

### 🤝 Créditos

- Desarrollado por [**@R3LI4NT**](https://github.com/R3LI4NT)
- Construido sobre el [Tor Project](https://www.torproject.org/) — este proyecto no está afiliado ni respaldado por el Tor Project
- Usa [egui](https://github.com/emilk/egui) para la GUI, [Phosphor Icons](https://phosphoricons.com/) para la iconografía, [rodio](https://github.com/RustAudio/rodio) para el audio sintetizado

### 📜 Licencia

MIT. Ver [LICENSE](LICENSE).

---

<p align="center">
  <sub>Made with ☕ and Rust · Hecho con ☕ y Rust</sub>
</p>

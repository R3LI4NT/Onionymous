<p align="center">
  <img width="265" height="260" alt="Onionymous" src="https://github.com/user-attachments/assets/6cab7c7d-9f99-4381-8671-832d3772747a" />
</p>

<h1 align="center">Onionymous</h1>

<p align="center">
  <em>Privacy-first desktop app that routes your traffic through the Tor network.</em><br/>
  <em>Aplicación de escritorio que enruta tu tráfico a través de la red Tor.</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-8a5cff?style=flat-square" alt="version" />
  <img src="https://img.shields.io/badge/platform-Windows%2010%2B-0078d4?style=flat-square" alt="platform" />
  <img src="https://img.shields.io/badge/built%20with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white" alt="rust" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="license" /></a>
</p>

<p align="center">
  <a href="#english">English</a> · <a href="#español">Español</a>
</p>

---

<a id="english"></a>

## English

### Overview

Onionymous is a native Windows desktop application that routes your network traffic through the Tor network, giving you anonymity, censorship circumvention, and the ability to present an IP from a country of your choice. It bundles the official Tor binary and geoip databases inside a single portable `.exe` — no separate installation, no external folders, no configuration files to edit by hand.

The goal is to offer a modern, visually polished experience for users who want the privacy benefits of Tor without the complexity of installing the Tor Browser or configuring a SOCKS proxy manually. Onionymous takes care of the Tor process, monitors bootstrap progress, rotates identities on demand, and gives you clear visual feedback on connection state and bandwidth.

### Features (v1.0.0)

**Connection control**

- One-click connect/disconnect — the central animated orb is the control, no separate button
- Live bootstrap progress shown as a filled progress arc around the orb
- Pulsating glow while connected, directional chirps when toggling state
- Public IP lookup on connect, displayed in amber on the dashboard for maximum visibility
- Automatic cleanup of stale Tor processes that would otherwise block the SOCKS port

**Routing**

- **Proxy mode** (default): Tor runs as a local SOCKS5 listener on `127.0.0.1:9050`. Apps that honour system proxy settings get routed; others don't
- **Exit location selector**: choose any of 40+ countries for your exit relay, or leave it on *Automatic*
- **Bridges / pluggable transports**: obfs4, meek-azure, snowflake, or custom bridge lines for censored networks
- **Excluded countries**: blacklist specific countries from ever being used as guard/middle/exit
- **Smart Connect**: auto-selects the best connection strategy based on your network profile

**Observability**

- Real-time bandwidth sparkline with violet download / amber upload, 60-sample rolling window
- Session totals for bytes downloaded and uploaded
- Full Tor + app event log with filtering by source and level, auto-scroll to tail
- Circuit summary card showing mode, exit country, and bridge status

**Tools**

- **Update Tor** button — downloads the latest Tor Expert Bundle directly from the official Tor Project archive, verifies its SHA-256 checksum against the signed manifest, and installs the new binaries atomically. Live terminal output with progress, colour-coded by severity
- Installed version detection with timestamp of last update

**UX**

- Purple gradient theme with DWM Mica/Acrylic blur on Windows 10/11
- Custom animated circular status indicator as the focal UI element
- Phosphor icon font throughout for consistent iconography
- Synthesized UI sounds (click, tick, connect chirp, disconnect chirp, error warble) generated in-memory — no audio files bundled
- Bilingual interface: English and Spanish, switchable at runtime with a globe button
- Portable executable: the `.exe` can be moved anywhere and continues to work. First launch extracts the embedded Tor runtime to `%LocalAppData%\Onionymous\runtime\<version>\`

**System integration**

- Start with Windows (via `HKCU\...\Run` registry entry, idempotent)
- Start minimized when launched via autostart
- Optional "minimize to tray on close" (off by default)
- Optional kill-switch firewall rules (Windows Defender, requires admin)
- Exit button for clean shutdown with automatic Tor disconnection

### What Onionymous is not

Be upfront about the limits so you know what you're getting:

- **It is not a full VPN.** Apps that ignore the system proxy or bind directly to network interfaces will not be routed through Tor. A full TUN/VPN mode is planned but not in v1.0.0
- **It is not the Tor Browser.** Onionymous routes traffic, but it doesn't give you browser-level anti-fingerprinting protections. For web browsing where anonymity really matters, use the Tor Browser
- **It is Windows-only for now.** The codebase is Rust and most of it is platform-agnostic, but the packaging, icon handling, autostart, and firewall integration are Windows-specific

### Installation

Download `onionymous.exe` from the [Releases](https://github.com/R3LI4NT/Onionymous/releases) page and run it. That's it — the `.exe` is fully self-contained.

### Building from source

You need [Rust 1.78+](https://rustup.rs) and `tar.exe` (ships with Windows 10 1803+).

```powershell
# Fetch the Tor Expert Bundle once — must run BEFORE cargo build,
# because the binaries are embedded into the .exe at compile time.
.\download-requeriments.bat

# Build the portable release executable.
cargo build --release
```

The resulting `target\release\onionymous.exe` is around 70 MB and fully portable. Move it anywhere.

For faster iteration during development:

```powershell
cargo build --no-default-features
Copy-Item -Recurse -Force .\resources .\target\debug\resources
```

### Credits

Developed by [@R3LI4NT](https://github.com/R3LI4NT).

Built on top of the [Tor Project](https://www.torproject.org/). This project is not affiliated with or endorsed by the Tor Project.

### License

MIT. See [LICENSE](LICENSE).

---

<a id="español"></a>

## Español

### Resumen

Onionymous es una aplicación nativa de escritorio para Windows que enruta tu tráfico de red a través de la red Tor, brindándote anonimato, capacidad para esquivar censura, y la posibilidad de presentar una IP del país que elijas. El binario oficial de Tor y las bases de datos geoip vienen embebidas dentro de un único `.exe` portable — sin instaladores, sin carpetas aparte, sin archivos de configuración que editar a mano.

El objetivo es ofrecer una experiencia moderna y visualmente cuidada a quienes quieren los beneficios de privacidad que brinda Tor sin la complejidad de instalar el Tor Browser o configurar un proxy SOCKS manualmente. Onionymous maneja el proceso de Tor, monitorea el bootstrap, rota identidades cuando lo pedís, y te muestra feedback visual claro sobre el estado de conexión y el ancho de banda.

### Características (v1.0.0)

**Control de conexión**

- Conectar/desconectar con un click — el orb central animado es el control, no hay botón aparte
- Progreso de bootstrap en vivo mostrado como un arco que se llena alrededor del orb
- Glow pulsante cuando está conectado, chirp direccional al cambiar de estado
- Consulta de IP pública al conectar, mostrada en ámbar sobre el dashboard para máxima visibilidad
- Limpieza automática de procesos Tor zombi que ocuparían el puerto SOCKS

**Enrutamiento**

- **Modo Proxy** (por defecto): Tor corre como listener SOCKS5 local en `127.0.0.1:9050`. Las apps que respetan el proxy del sistema se enrutan; las que no, no
- **Selector de país de salida**: elegí entre 40+ países para tu relay de salida, o dejalo en *Automático*
- **Puentes / pluggable transports**: obfs4, meek-azure, snowflake, o líneas de puentes personalizadas para redes censuradas
- **Países excluidos**: vetá países específicos para que nunca se usen como guard/middle/exit
- **Smart Connect**: elige automáticamente la mejor estrategia de conexión según tu perfil de red

**Observabilidad**

- Sparkline de ancho de banda en tiempo real con descarga en violeta y subida en ámbar, ventana rolante de 60 muestras
- Totales de sesión para bytes descargados y subidos
- Registro completo de eventos de Tor + app con filtros por origen y nivel, auto-scroll al pie
- Card de resumen del circuito: modo, país de salida, estado de puentes

**Herramientas**

- Botón **Actualizar Tor** — descarga el Tor Expert Bundle más reciente directamente del archivo oficial del Tor Project, verifica su checksum SHA-256 contra el manifiesto firmado, e instala los binarios nuevos de forma atómica. Terminal con progreso en vivo, colores por nivel de severidad
- Detección de la versión instalada con timestamp de la última actualización

**Experiencia de usuario**

- Tema con gradiente violeta y blur Mica/Acrylic de DWM en Windows 10/11
- Indicador circular animado propio como elemento focal de la UI
- Iconos de la fuente Phosphor en toda la app para consistencia visual
- Sonidos UI sintetizados en memoria (click, tick, chirp de conexión, chirp de desconexión, warble de error) — ningún archivo de audio incluido
- Interfaz bilingüe: inglés y español, alternable en tiempo real con el botón del globo
- Ejecutable portable: el `.exe` se puede mover a cualquier carpeta y seguir funcionando. En el primer arranque extrae el runtime de Tor embebido a `%LocalAppData%\Onionymous\runtime\<versión>\`

**Integración con el sistema**

- Iniciar con Windows (entrada en `HKCU\...\Run` del registro, idempotente)
- Iniciar minimizado cuando arranca por autostart
- Opcional "minimizar a la bandeja al cerrar" (desactivado por defecto)
- Reglas de kill-switch opcionales para el firewall (Windows Defender, requiere admin)
- Botón Salir para cierre limpio con desconexión automática de Tor

### Qué no es Onionymous

Vale la pena ser claro con los límites:

- **No es una VPN completa.** Las apps que ignoren el proxy del sistema o se conecten directamente a interfaces de red no se van a enrutar por Tor. Un modo TUN/VPN completo está planeado pero no está en la v1.0.0
- **No es el Tor Browser.** Onionymous enruta tráfico, pero no te da las protecciones anti-fingerprinting a nivel navegador. Para navegación web donde el anonimato realmente importa, usá el Tor Browser
- **Solo Windows por ahora.** El código es Rust y la mayor parte es agnóstico a plataforma, pero el empaquetado, manejo de íconos, autostart e integración con el firewall son específicos de Windows

### Instalación

Descargá `onionymous.exe` de la página de [Releases](https://github.com/R3LI4NT/Onionymous/releases) y ejecutalo. Eso es todo — el `.exe` es totalmente autónomo.

### Compilar desde el código fuente

Necesitás [Rust 1.78+](https://rustup.rs) y `tar.exe` (viene con Windows 10 1803+).

```powershell
# Descargar el Tor Expert Bundle una sola vez — se debe ejecutar
# ANTES de cargo build, porque los binarios se embeben en el .exe
# en tiempo de compilación.
.\download-requeriments.bat

# Build del ejecutable portable en release.
cargo build --release
```

El `target\release\onionymous.exe` resultante pesa alrededor de 70 MB y es totalmente portable. Movelo a cualquier lado.

Para iteración más rápida durante desarrollo:

```powershell
cargo build --no-default-features
Copy-Item -Recurse -Force .\resources .\target\debug\resources
```

### Créditos

Desarrollado por [@R3LI4NT](https://github.com/R3LI4NT).

Construido sobre el [Tor Project](https://www.torproject.org/). Este proyecto no está afiliado ni respaldado por el Tor Project.

### Licencia

MIT. Ver [LICENSE](LICENSE).

---

<p align="center">
  <sub>Made with ☕ and Rust</sub>
</p>

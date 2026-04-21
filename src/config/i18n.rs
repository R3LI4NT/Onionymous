use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Language::English => "EN",
            Language::Spanish => "ES",
        }
    }

    pub fn all() -> &'static [Language] {
        &[Language::English, Language::Spanish]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
    NavDashboard,
    NavSettings,
    NavLogs,
    NavAbout,

    HeaderTitle,
    StatusDisconnected,
    StatusConnecting,
    StatusConnected,
    StatusDisconnecting,
    StatusFailed,
    StatusDescDisconnected,
    StatusDescConnected,
    StatusDescDisconnecting,

    Connect,
    Disconnect,
    Working,
    NewIdentity,
    RefreshIp,
    CurrentIp,

    Routing,
    SmartConnect,
    SmartConnectDesc,
    Mode,
    ExitLocation,
    Automatic,

    TorNetwork,
    Download,
    Upload,
    SessionTotal,
    TrafficHint,

    SettingsTitle,
    AppBehaviour,
    StartWithSystem,
    StartMinimized,
    MinimizeToTray,
    BridgesTitle,
    BridgesEnable,
    BridgesTransport,
    BridgesCustom,
    Security,
    KillSwitch,
    Ports,
    SocksPort,
    ControlPort,
    DnsPort,
    ExcludedCountries,
    ExcludedDesc,

    LogsTitle,
    LatestEvents,
    Clear,
    NoLogs,

    AboutTitle,
    AboutBlurb,
    Credits,
    Disclaimer,
    DisclaimerText,
    Version,

    Language,

    Tools,
    TorRuntime,
    UpdateTor,
    UpdateTorHint,
    Updating,
    CurrentVersion,
    LastChecked,
    NeverUpdated,
    EmbeddedBundle,
    UpdateOutput,

    Exit,
}

pub fn t(lang: Language, key: Key) -> &'static str {
    match lang {
        Language::English => en(key),
        Language::Spanish => es(key),
    }
}

fn en(key: Key) -> &'static str {
    use Key::*;
    match key {
        NavDashboard => "Dashboard",
        NavSettings => "Settings",
        NavLogs => "Logs",
        NavAbout => "About",

        HeaderTitle => "Dashboard",
        StatusDisconnected => "Disconnected",
        StatusConnecting => "Connecting…",
        StatusConnected => "Connected",
        StatusDisconnecting => "Disconnecting…",
        StatusFailed => "Connection failed",
        StatusDescDisconnected => "Ready to route traffic through Tor.",
        StatusDescConnected => "Your traffic is routed anonymously through Tor.",
        StatusDescDisconnecting => "Tearing down the connection…",

        Connect => "Connect",
        Disconnect => "Disconnect",
        Working => "Working…",
        NewIdentity => "New Identity",
        RefreshIp => "Refresh IP",
        CurrentIp => "Current IP",

        Routing => "Routing",
        SmartConnect => "Smart Connect (recommended)",
        SmartConnectDesc => "Automatically picks the best Tor connection strategy for your \
                             network and country profile. When enabled, bridge/censorship \
                             settings may be overridden.",
        Mode => "Mode",
        ExitLocation => "Exit location",
        Automatic => "Automatic",

        TorNetwork => "Tor Network",
        Download => "Download",
        Upload => "Upload",
        SessionTotal => "Session total",
        TrafficHint => "Shows Tor's own traffic (from the control port).",

        SettingsTitle => "Settings",
        AppBehaviour => "Application behaviour",
        StartWithSystem => "Start with Windows",
        StartMinimized => "Start minimized",
        MinimizeToTray => "Minimize to tray on close",
        BridgesTitle => "Bridges / pluggable transports",
        BridgesEnable => "Enable bridges (for censored networks)",
        BridgesTransport => "Transport type",
        BridgesCustom => "Custom bridge lines (one per line)",
        Security => "Security",
        KillSwitch => "Kill Switch (block all non-Tor traffic at the firewall level)",
        Ports => "Ports",
        SocksPort => "SOCKS port",
        ControlPort => "Control port",
        DnsPort => "DNS port",
        ExcludedCountries => "Excluded exit countries",
        ExcludedDesc => "Tick the countries you never want to route traffic through.",

        LogsTitle => "Logs",
        LatestEvents => "Latest events",
        Clear => "Clear",
        NoLogs => "No log entries yet. Try connecting.",

        AboutTitle => "About",
        AboutBlurb => "Onionymous is a privacy-first desktop application that routes your \
                       traffic through the Tor network via a local SOCKS5 proxy. \
                       Includes pluggable transports (obfs4, Snowflake) for \
                       censored networks, and country-level exit node selection.",
        Credits => "Credits",
        Disclaimer => "Disclaimer",
        DisclaimerText => "Tor usage can be restricted or illegal in some jurisdictions. \
                           You are responsible for complying with local laws and regulations. \
                           Onionymous is provided as-is without warranty of any kind.",
        Version => "Version",

        Language => "Language",

        Tools => "Tools",
        TorRuntime => "Tor runtime",
        UpdateTor => "Update Tor",
        UpdateTorHint => "Downloads the latest Tor Expert Bundle from the official Tor Project archive, verifies its SHA-256 checksum, and installs it over the embedded copy.",
        Updating => "Updating…",
        CurrentVersion => "Current version",
        LastChecked => "Last updated",
        NeverUpdated => "never (using embedded copy)",
        EmbeddedBundle => "embedded bundle",
        UpdateOutput => "Update output",

        Exit => "Exit",
    }
}

fn es(key: Key) -> &'static str {
    use Key::*;
    match key {
        NavDashboard => "Panel",
        NavSettings => "Ajustes",
        NavLogs => "Registros",
        NavAbout => "Acerca de",

        HeaderTitle => "Panel",
        StatusDisconnected => "Desconectado",
        StatusConnecting => "Conectando…",
        StatusConnected => "Conectado",
        StatusDisconnecting => "Desconectando…",
        StatusFailed => "Conexión fallida",
        StatusDescDisconnected => "Listo para enrutar tráfico a través de Tor.",
        StatusDescConnected => "Tu tráfico se enruta anónimamente a través de Tor.",
        StatusDescDisconnecting => "Cerrando la conexión…",

        Connect => "Conectar",
        Disconnect => "Desconectar",
        Working => "Procesando…",
        NewIdentity => "Nueva identidad",
        RefreshIp => "Actualizar IP",
        CurrentIp => "IP actual",

        Routing => "Enrutamiento",
        SmartConnect => "Conexión inteligente (recomendada)",
        SmartConnectDesc => "Elige automáticamente la mejor estrategia de conexión a Tor \
                             según tu red y país. Al activarla, la configuración de \
                             puentes y anticensura puede ser ignorada.",
        Mode => "Modo",
        ExitLocation => "País de salida",
        Automatic => "Automático",

        TorNetwork => "Red Tor",
        Download => "Descarga",
        Upload => "Subida",
        SessionTotal => "Total de sesión",
        TrafficHint => "Muestra el tráfico de Tor (puerto de control).",

        SettingsTitle => "Ajustes",
        AppBehaviour => "Comportamiento de la aplicación",
        StartWithSystem => "Iniciar con Windows",
        StartMinimized => "Iniciar minimizado",
        MinimizeToTray => "Minimizar a la bandeja al cerrar",
        BridgesTitle => "Puentes / transportes conectables",
        BridgesEnable => "Activar puentes (para redes censuradas)",
        BridgesTransport => "Tipo de transporte",
        BridgesCustom => "Líneas de puente personalizadas (una por línea)",
        Security => "Seguridad",
        KillSwitch => "Kill Switch (bloquea todo el tráfico no-Tor a nivel firewall)",
        Ports => "Puertos",
        SocksPort => "Puerto SOCKS",
        ControlPort => "Puerto de control",
        DnsPort => "Puerto DNS",
        ExcludedCountries => "Países de salida excluidos",
        ExcludedDesc => "Marca los países por los que nunca querés enrutar tu tráfico.",

        LogsTitle => "Registros",
        LatestEvents => "Últimos eventos",
        Clear => "Limpiar",
        NoLogs => "Todavía no hay registros. Probá conectarte.",

        AboutTitle => "Acerca de",
        AboutBlurb => "Onionymous es una aplicación de escritorio centrada en la privacidad \
                       que enruta tu tráfico a través de la red Tor mediante un proxy SOCKS5 \
                       local. Incluye transportes conectables (obfs4, Snowflake) \
                       para redes censuradas y selección de nodo de salida por país.",
        Credits => "Créditos",
        Disclaimer => "Aviso legal",
        DisclaimerText => "El uso de Tor puede estar restringido o ser ilegal en algunas \
                           jurisdicciones. Eres responsable de cumplir con las leyes y \
                           regulaciones locales. Onionymous se proporciona tal cual, sin \
                           garantía de ningún tipo.",
        Version => "Versión",

        Language => "Idioma",

        Tools => "Herramientas",
        TorRuntime => "Runtime de Tor",
        UpdateTor => "Actualizar Tor",
        UpdateTorHint => "Descarga el último Tor Expert Bundle desde el archivo oficial del Tor Project, verifica su checksum SHA-256, y lo instala sobre la copia embebida.",
        Updating => "Actualizando…",
        CurrentVersion => "Versión actual",
        LastChecked => "Última actualización",
        NeverUpdated => "nunca (usando copia embebida)",
        EmbeddedBundle => "paquete embebido",
        UpdateOutput => "Salida de la actualización",

        Exit => "Salir",
    }
}

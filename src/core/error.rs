use thiserror::Error;

#[derive(Debug, Error)]
pub enum OnionymousError {
    #[error("Tor process error: {0}")]
    TorProcess(String),

    #[error("Tor control port error: {0}")]
    TorControl(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Routing error: {0}")]
    Routing(String),

    #[error("Permission denied: {0} — administrator privileges required")]
    PermissionDenied(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, OnionymousError>;

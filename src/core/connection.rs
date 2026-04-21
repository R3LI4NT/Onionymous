use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting { progress: u8, stage: String },
    Connected,
    Failed(String),
    Disconnecting,
}

impl ConnectionStatus {
    pub fn short_label(&self) -> String {
        match self {
            ConnectionStatus::Disconnected => "Disconnected".into(),
            ConnectionStatus::Connecting { progress, .. } => format!("Connecting… {}%", progress),
            ConnectionStatus::Connected => "Connected".into(),
            ConnectionStatus::Failed(_) => "Connection failed".into(),
            ConnectionStatus::Disconnecting => "Disconnecting…".into(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            ConnectionStatus::Disconnected => "Ready to route traffic through Tor.".into(),
            ConnectionStatus::Connecting { stage, progress } => {
                format!("{} — {}%", stage, progress)
            }
            ConnectionStatus::Connected => "Your traffic is routed anonymously through Tor.".into(),
            ConnectionStatus::Failed(msg) => msg.clone(),
            ConnectionStatus::Disconnecting => "Tearing down the connection…".into(),
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionStatus::Connected)
    }

    pub fn is_transitioning(&self) -> bool {
        matches!(
            self,
            ConnectionStatus::Connecting { .. } | ConnectionStatus::Disconnecting
        )
    }
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Disconnected
    }
}

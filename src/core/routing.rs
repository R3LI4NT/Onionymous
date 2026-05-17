use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingMode {
    Proxy,
    Tun,
}

impl Default for RoutingMode {
    fn default() -> Self {
        RoutingMode::Proxy
    }
}

impl RoutingMode {
    pub fn label(&self) -> &'static str {
        match self {
            RoutingMode::Proxy => "Proxy (SOCKS5)",
            RoutingMode::Tun => "TUN/VPN (Admin)",
        }
    }

    pub fn requires_admin(&self) -> bool {
        matches!(self, RoutingMode::Tun)
    }

    pub fn all() -> &'static [RoutingMode] {
        &[RoutingMode::Proxy, RoutingMode::Tun]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BridgeType {
    #[default]
    Obfs4,
    Custom,
}

impl BridgeType {
    pub fn label(&self) -> &'static str {
        match self {
            BridgeType::Obfs4 => "obfs4 (recomendado)",
            BridgeType::Custom => "Puentes personalizados",
        }
    }

    pub fn all() -> &'static [BridgeType] {
        &[BridgeType::Obfs4, BridgeType::Custom]
    }
}

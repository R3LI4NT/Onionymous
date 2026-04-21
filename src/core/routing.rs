use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingMode {
    Proxy,
}

impl Default for RoutingMode {
    fn default() -> Self {
        RoutingMode::Proxy
    }
}

impl RoutingMode {
    pub fn label(&self) -> &'static str {
        match self {
            RoutingMode::Proxy => "Proxy Mode (SOCKS5)",
        }
    }

    pub fn requires_admin(&self) -> bool {
        false
    }

    pub fn all() -> &'static [RoutingMode] {
        &[RoutingMode::Proxy]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BridgeType {

    #[default]
    Obfs4,
    Snowflake,
    Conjure,
    Custom,
}

impl BridgeType {
    pub fn label(&self) -> &'static str {
        match self {
            BridgeType::Obfs4 => "obfs4 (recommended)",
            BridgeType::Snowflake => "Snowflake",
            BridgeType::Conjure => "Conjure (experimental)",
            BridgeType::Custom => "Custom bridges",
        }
    }

    pub fn all() -> &'static [BridgeType] {
        &[
            BridgeType::Obfs4,
            BridgeType::Snowflake,
            BridgeType::Conjure,
            BridgeType::Custom,
        ]
    }
}
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Configuration for an IP endpoint
pub struct IPEndpointConfig {
    /// Bind/Connect address of the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// Port of the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Configuration for a serial port endpoint
pub struct SerialPortEndpointConfig {
    /// Serial port name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Baud rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baud_rate: Option<u32>,
}

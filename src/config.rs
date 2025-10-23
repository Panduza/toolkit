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
pub struct UsbEndpointConfig {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vid: Option<u16>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u16>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Configuration for a serial port endpoint
pub struct SerialPortEndpointConfig {
    /// Serial port name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// USB endpoint configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb: Option<UsbEndpointConfig>,

    /// Baud rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baud_rate: Option<u32>,
}

// =============================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Configuration for a broker
pub struct BrokerConfig {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<IPEndpointConfig>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<IPEndpointConfig>,
}

impl Default for BrokerConfig {
    fn default() -> Self {
        Self {
            tcp: Some(IPEndpointConfig {
                addr: Some("0.0.0.0".into()),
                port: Some(1883),
            }),
            websocket: Some(IPEndpointConfig {
                addr: Some("0.0.0.0".into()),
                port: Some(8083),
            }),
        }
    }
}

use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{error, info};

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
pub struct MqttBrokerConfig {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<IPEndpointConfig>,

    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<IPEndpointConfig>,
}

impl Default for MqttBrokerConfig {
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

// =============================================================

pub fn write_config<T>(config_path: &Path, config_obj: &T)
where
    T: ?Sized + Serialize,
{
    // Serialize to JSON format with pretty printing
    let config_content = serde_json::to_string_pretty(&config_obj)
        .expect("Failed to serialize default configuration");

    // Write the configuration file
    if let Err(err) = std::fs::write(config_path, config_content) {
        error!("Failed to write default configuration file: {}", err);
    } else {
        info!(
            "Generated default configuration file at: {}",
            config_path.display()
        );
    }
}

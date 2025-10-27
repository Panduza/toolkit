use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{error, info};

//------------------------------------------------------------------------------

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
/// Configuration for a USB endpoint
pub struct UsbEndpointConfig {
    /// USB Vendor ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vid: Option<u16>,

    /// USB Product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u16>,

    /// USB Serial number
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
    /// TCP endpoint configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<IPEndpointConfig>,

    /// WebSocket endpoint configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<IPEndpointConfig>,
}

//------------------------------------------------------------------------------

impl MqttBrokerConfig {
    pub fn new_for_meduse() -> Self {
        Self {
            tcp: Some(IPEndpointConfig {
                addr: Some("12.0.0.1".into()),
                port: Some(1883),
            }),
            websocket: Some(IPEndpointConfig {
                addr: Some("0.0.0.0".into()),
                port: Some(8083),
            }),
        }
    }
}

//------------------------------------------------------------------------------

impl Default for MqttBrokerConfig {
    fn default() -> Self {
        Self {
            tcp: Some(IPEndpointConfig {
                addr: Some("127.0.0.1".into()),
                port: Some(1883),
            }),
            websocket: None,
        }
    }
}

// =============================================================

//------------------------------------------------------------------------------

/// Write configuration in JSON5 format with hex numbers for USB IDs
pub fn write_config<T>(config_path: &Path, config_obj: &T)
where
    T: Serialize,
{
    // Serialize to JSON5 format
    let config_content = serde_json::to_string_pretty(config_obj)
        .expect("Failed to serialize configuration to JSON5");

    // Post-process to make USB IDs appear as hex
    let config_content = format_usb_ids_as_hex(&config_content);

    // Write the configuration file
    if let Err(err) = std::fs::write(config_path, config_content) {
        error!("Failed to write JSON5 configuration file: {}", err);
    } else {
        info!(
            "Generated JSON5 configuration file at: {}",
            config_path.display()
        );
    }
}

//------------------------------------------------------------------------------

/// Format USB vendor and product IDs as hexadecimal in JSON5 string
fn format_usb_ids_as_hex(json5_content: &str) -> String {
    use regex::Regex;

    // Replace vid and pid numeric values with hex format
    let re_vid = Regex::new(r#""vid":\s*(\d+)"#).unwrap();
    let re_pid = Regex::new(r#""pid":\s*(\d+)"#).unwrap();

    let content = re_vid.replace_all(json5_content, |caps: &regex::Captures| {
        let num: u16 = caps[1].parse().unwrap_or(0);
        format!(r#""vid": 0x{:04X}"#, num)
    });

    let content = re_pid.replace_all(&content, |caps: &regex::Captures| {
        let num: u16 = caps[1].parse().unwrap_or(0);
        format!(r#""pid": 0x{:04X}"#, num)
    });

    content.to_string()
}

//------------------------------------------------------------------------------

/// Read configuration from JSON5 format
///
/// If the file does not exist or if the file is empty:
///     - create a default configuration
///     - create the file and return.
///
/// If the file is malformed, an error is returned.
///
/// This function is important for user feedback, so it uses info logs to report its steps.:
///
/// If error this function returns a ConfigError with details.
///
pub fn read_config<T>(config_path: &Path) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de> + Default + Serialize,
{
    match std::fs::read_to_string(config_path) {
        Ok(content) => {
            // Check if the file is empty or contains only whitespace
            if content.trim().is_empty() {
                info!(
                    "Configuration file is empty: {}, creating default configuration",
                    config_path.display()
                );
                let default_config = T::default();
                write_config(config_path, &default_config);
                Ok(default_config)
            } else {
                info!("Reading configuration from: {}", config_path.display());
                // File has content, try to parse it
                let config: T = serde_json5::from_str(&content)?;
                info!(
                    "Successfully loaded configuration from: {}",
                    config_path.display()
                );
                Ok(config)
            }
        }
        Err(_) => {
            error!(
                "Configuration file does not exist: {}, creating default configuration",
                config_path.display()
            );
            let default_config = T::default();
            write_config(config_path, &default_config);
            Ok(default_config)
        }
    }
}

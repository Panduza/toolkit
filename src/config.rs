use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::Path;
use tracing::{error, info};

//------------------------------------------------------------------------------

/// Serialize u16 as hex string
fn serialize_u16_as_hex<S>(value: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&format!("0x{:04X}", v)),
        None => serializer.serialize_none(),
    }
}

/// Deserialize hex string or number to u16
fn deserialize_u16_from_hex<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Number(n) => {
            if let Some(num) = n.as_u64() {
                if num <= u16::MAX as u64 {
                    Ok(Some(num as u16))
                } else {
                    Err(D::Error::custom("Number too large for u16"))
                }
            } else {
                Err(D::Error::custom("Invalid number"))
            }
        }
        serde_json::Value::String(s) => {
            if s.starts_with("0x") || s.starts_with("0X") {
                u16::from_str_radix(&s[2..], 16)
                    .map(Some)
                    .map_err(|_| D::Error::custom("Invalid hex string"))
            } else {
                s.parse::<u16>()
                    .map(Some)
                    .map_err(|_| D::Error::custom("Invalid number string"))
            }
        }
        _ => Err(D::Error::custom("Expected number or hex string")),
    }
}

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
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_u16_as_hex",
        deserialize_with = "deserialize_u16_from_hex"
    )]
    pub vid: Option<u16>,

    /// USB Product ID
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_u16_as_hex",
        deserialize_with = "deserialize_u16_from_hex"
    )]
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

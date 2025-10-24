use crate::config::BrokerConfig;
use crate::config::IPEndpointConfig;
use config::Map;
use config::Value;
use rumqttd::Broker;
use rumqttd::Config;
use tracing::info;

/// Start the MQTT broker in a separate thread
#[deprecated(since = "0.1.0", note = "Use start_broker instead")]
pub fn start(ip_endpoint: &IPEndpointConfig) -> std::thread::JoinHandle<()> {
    //
    // info
    info!("----- SERVICE : START BROKER -----");

    let host: &str = ip_endpoint.addr.as_ref().unwrap();
    let port = ip_endpoint.port.unwrap();
    let listen_addr = format!("{}:{}", host, port);

    let mut router: std::collections::HashMap<String, Value> = Map::new();
    router.insert("id".to_string(), Value::new(None, 0));
    router.insert("max_connections".to_string(), Value::new(None, 20480));
    router.insert(
        "max_outgoing_packet_count".to_string(),
        Value::new(None, 200),
    );
    router.insert("max_segment_size".to_string(), Value::new(None, 104857600));
    router.insert("max_segment_count".to_string(), Value::new(None, 10));

    let mut server_connections: std::collections::HashMap<String, config::Value> = Map::new();
    server_connections.insert("connection_timeout_ms".to_string(), Value::new(None, 60000));
    server_connections.insert("max_payload_size".to_string(), Value::new(None, 2000480));
    server_connections.insert("max_inflight_count".to_string(), Value::new(None, 20480));
    server_connections.insert("dynamic_filters".to_string(), Value::new(None, true));

    let mut server: std::collections::HashMap<String, Value> = Map::new();
    server.insert("name".to_string(), Value::new(None, "v4-1"));
    server.insert("listen".to_string(), Value::new(None, listen_addr.clone()));
    server.insert("next_connection_delay_ms".to_string(), Value::new(None, 1));
    server.insert(
        "connections".to_string(),
        Value::new(None, server_connections),
    );

    // see docs of config crate to know more
    let config = config::Config::builder()
        .set_default("id", 0)
        .unwrap()
        .set_default("router", router)
        .unwrap()
        .set_default("v4.1", server)
        .unwrap()
        .build()
        .unwrap();

    //
    // this is where we deserialize it into Config
    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);

    //
    // start broker
    info!("Broker listen on: {}", listen_addr);
    let _jh = std::thread::spawn(move || {
        broker.start().unwrap();
        println!("BROKER STOPPPED !!!!!!!!!!!!!!!!!");
    });

    return _jh;
}

//------------------------------------------------------------------------------

/// WebSocket configuration section
///
/// [ws.1]
/// name = "ws-1"
/// listen = "0.0.0.0:8083"
/// next_connection_delay_ms = 1
///     [ws.1.connections]
///     connection_timeout_ms = 60000
///     max_client_id_len = 256
///     throttle_delay_ms = 0
///     max_payload_size = 20480
///     max_inflight_count = 500
///     max_inflight_size = 1024
///
pub fn websocket_section(broker_config: &BrokerConfig) -> std::collections::HashMap<String, Value> {
    // extract host and port
    let host: &str = broker_config
        .websocket
        .as_ref()
        .unwrap()
        .addr
        .as_ref()
        .unwrap();
    let port = broker_config.websocket.as_ref().unwrap().port.unwrap();
    let listen_addr = format!("{}:{}", host, port);

    // Connections settings
    let mut connections: std::collections::HashMap<String, config::Value> = Map::new();
    connections.insert("connection_timeout_ms".to_string(), Value::new(None, 60000));
    connections.insert("max_client_id_len".to_string(), Value::new(None, 256));
    connections.insert("throttle_delay_ms".to_string(), Value::new(None, 0));
    connections.insert("max_payload_size".to_string(), Value::new(None, 20480));
    connections.insert("max_inflight_count".to_string(), Value::new(None, 500));
    connections.insert("max_inflight_size".to_string(), Value::new(None, 1024));

    // Server settings
    let mut ws: std::collections::HashMap<String, Value> = Map::new();
    ws.insert("name".to_string(), Value::new(None, "ws-1"));
    ws.insert("listen".to_string(), Value::new(None, listen_addr.clone()));
    ws.insert("next_connection_delay_ms".to_string(), Value::new(None, 1));
    ws.insert("connections".to_string(), Value::new(None, connections));

    // Logging
    info!("Broker listen on **ws**:{}", listen_addr);

    // return the object
    ws
}

//------------------------------------------------------------------------------

/// TCPv4 configuration section
///
pub fn tcpv4_section(broker_config: &BrokerConfig) -> std::collections::HashMap<String, Value> {
    // extract host and port
    let host: &str = broker_config.tcp.as_ref().unwrap().addr.as_ref().unwrap();
    let port = broker_config.tcp.as_ref().unwrap().port.unwrap();
    let listen_addr = format!("{}:{}", host, port);

    // Connections settings
    let mut connections: std::collections::HashMap<String, config::Value> = Map::new();
    connections.insert("connection_timeout_ms".to_string(), Value::new(None, 60000));
    connections.insert("max_payload_size".to_string(), Value::new(None, 20480));
    connections.insert("max_inflight_count".to_string(), Value::new(None, 20480));
    connections.insert("dynamic_filters".to_string(), Value::new(None, true));

    // Server settings
    let mut tcp: std::collections::HashMap<String, Value> = Map::new();
    tcp.insert("name".to_string(), Value::new(None, "v4-1"));
    tcp.insert("listen".to_string(), Value::new(None, listen_addr.clone()));
    tcp.insert("next_connection_delay_ms".to_string(), Value::new(None, 1));
    tcp.insert("connections".to_string(), Value::new(None, connections));

    // Logging
    info!("Broker listen on **tcp**:{}", listen_addr);

    // return the object
    tcp
}

//------------------------------------------------------------------------------

/// Start the broker
/// This function will start the MQTT broker with the given configuration.
pub fn start_broker(broker_config: &BrokerConfig) -> std::thread::JoinHandle<()> {
    //
    // info
    info!("----- SERVICE : START BROKER -----");

    let mut router: std::collections::HashMap<String, Value> = Map::new();
    router.insert("id".to_string(), Value::new(None, 0));
    router.insert("max_connections".to_string(), Value::new(None, 20480));
    router.insert(
        "max_outgoing_packet_count".to_string(),
        Value::new(None, 200),
    );
    router.insert("max_segment_size".to_string(), Value::new(None, 104857600));
    router.insert("max_segment_count".to_string(), Value::new(None, 10));

    // see docs of config crate to know more
    let mut config_builder = config::Config::builder()
        .set_default("id", 0)
        .unwrap()
        .set_default("router", router)
        .unwrap();

    // Only add TCP section if tcp config is present
    if broker_config.tcp.is_some() {
        config_builder = config_builder
            .set_default("v4.1", tcpv4_section(broker_config))
            .unwrap();
    }

    // Only add WebSocket section if websocket config is present
    if broker_config.websocket.is_some() {
        config_builder = config_builder
            .set_default("ws.1", websocket_section(broker_config))
            .unwrap();
    }

    let config = config_builder.build().unwrap();

    //
    // this is where we deserialize it into Config
    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);

    //
    // start broker
    let _jh = std::thread::spawn(move || {
        broker.start().unwrap();
        println!("BROKER STOPPPED !!!!!!!!!!!!!!!!!");
    });

    return _jh;
}

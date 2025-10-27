use crate::rand::generate_random_string;
use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;

/// MQTT initialization utilities
pub fn init_client<A: Into<String>>(module_name: A) -> (AsyncClient, rumqttc::EventLoop) {
    // Generate a unique client ID
    let client_id = format!("{}-{}", module_name.into(), generate_random_string(5));

    // Initialize MQTT client
    let mut mqttoptions = MqttOptions::new(client_id, "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(3));

    // Create the AsyncClient and EventLoop
    let (client, event_loop) = AsyncClient::new(mqttoptions, 100);
    (client, event_loop)
}

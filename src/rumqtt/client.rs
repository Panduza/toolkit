use crate::rand::generate_random_string;
use rumqttc::AsyncClient;
use rumqttc::MqttOptions;
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

#[derive(Clone)]
/// Custom wrapper around rumqttc::AsyncClient with predefined QoS and retain settings
pub struct RumqttCustomAsyncClient {
    /// The underlying MQTT asynchronous client
    pub client: AsyncClient,
    /// Quality of Service level for MQTT messages
    pub qos: rumqttc::QoS,
    /// Retain flag for MQTT messages
    pub retain: bool,

    pub prefix: String,
}

impl RumqttCustomAsyncClient {
    /// Create a new RumqttCustomAsyncClient with specified QoS and retain settings
    pub fn new(client: AsyncClient, qos: rumqttc::QoS, retain: bool, prefix: String) -> Self {
        Self {
            client,
            qos,
            retain,
            prefix,
        }
    }

    /// Subscribe to all relevant MQTT topics
    pub async fn subscribe_to_all(&self, topics: Vec<String>) {
        for topic in topics {
            self.client.subscribe(topic, self.qos).await.unwrap();
        }
    }

    /// Publish a message to a topic using the predefined QoS and retain settings
    pub async fn publish<A: Into<String>, V: Into<Vec<u8>>>(
        &self,
        topic: A,
        payload: V,
    ) -> Result<(), rumqttc::ClientError> {
        self.client
            .publish(topic.into(), self.qos, self.retain, payload)
            .await
    }

    /// Generate a topic string with the configured prefix
    pub fn topic_with_prefix<A: AsRef<str>>(&self, topic: A) -> String {
        format!("{}/{}", self.prefix, topic.as_ref())
    }
}

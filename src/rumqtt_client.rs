use rumqttc::AsyncClient;

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

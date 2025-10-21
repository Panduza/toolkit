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
}

impl RumqttCustomAsyncClient {
    /// Create a new RumqttCustomAsyncClient with specified QoS and retain settings
    pub fn new(client: AsyncClient, qos: rumqttc::QoS, retain: bool) -> Self {
        Self {
            client,
            qos,
            retain,
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
}

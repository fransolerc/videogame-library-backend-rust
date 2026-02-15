use async_trait::async_trait;
use serde_json::to_string;
use crate::application::ports::output::favorite_game_event_publisher::{FavoriteGameEvent, FavoriteGameEventPublisher};

// Dummy implementation that logs events instead of sending to Kafka
pub struct KafkaFavoriteGameEventPublisher {
    topic: String,
}

impl KafkaFavoriteGameEventPublisher {
    pub fn new(_bootstrap_servers: &str, topic: &str) -> Result<Self, String> {
        Ok(Self {
            topic: topic.to_string(),
        })
    }
}

#[async_trait]
impl FavoriteGameEventPublisher for KafkaFavoriteGameEventPublisher {
    async fn publish_favorite_game_event(&self, event: FavoriteGameEvent) -> Result<(), String> {
        let payload = to_string(&event).map_err(|e| format!("Serialization error: {}", e))?;

        // Just log the event instead of sending to Kafka
        tracing::info!("(MOCK KAFKA) Published to topic '{}': {}", self.topic, payload);

        Ok(())
    }
}

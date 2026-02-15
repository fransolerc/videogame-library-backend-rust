use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteGameEvent {
    pub user_id: Uuid,
    pub game_id: i64,
    pub is_favorite: bool,
}

#[async_trait]
pub trait FavoriteGameEventPublisher: Send + Sync {
    async fn publish_favorite_game_event(&self, event: FavoriteGameEvent) -> Result<(), String>;
}

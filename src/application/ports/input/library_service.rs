use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::{UserGame, GameStatus};

#[async_trait]
pub trait LibraryService: Send + Sync {
    async fn upsert_game_in_library(&self, user_id: Uuid, game_id: i64, status: GameStatus) -> Result<Option<UserGame>, String>;
    async fn list_user_library(&self, user_id: Uuid) -> Result<Vec<UserGame>, String>;
    async fn get_user_game_status(&self, user_id: Uuid, game_id: i64) -> Result<Option<UserGame>, String>;
    async fn remove_game_from_library(&self, user_id: Uuid, game_id: i64) -> Result<(), String>;
    async fn add_game_to_favorites(&self, user_id: Uuid, game_id: i64) -> Result<UserGame, String>;
    async fn remove_game_from_favorites(&self, user_id: Uuid, game_id: i64) -> Result<(), String>;
    // TODO: Define Pageable and Page structs
    async fn list_favorite_games(&self, user_id: Uuid, page: i32, size: i32) -> Result<Vec<UserGame>, String>;
}

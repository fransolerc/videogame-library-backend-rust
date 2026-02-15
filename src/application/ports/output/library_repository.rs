use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::UserGame;

#[async_trait]
pub trait LibraryRepository: Send + Sync {
    async fn save(&self, user_game: &UserGame) -> Result<UserGame, String>;
    async fn find_by_user_id_and_game_id(&self, user_id: Uuid, game_id: i64) -> Result<Option<UserGame>, String>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserGame>, String>;
    async fn update(&self, user_game: &UserGame) -> Result<UserGame, String>;
    async fn delete_by_user_id_and_game_id(&self, user_id: Uuid, game_id: i64) -> Result<(), String>;
    // TODO: Define Pageable and Page structs
    async fn find_by_user_id_and_is_favorite_true(&self, user_id: Uuid, page: i32, size: i32) -> Result<Vec<UserGame>, String>;
}

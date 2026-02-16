use async_trait::async_trait;
use crate::domain::game::Game;
use crate::domain::page::Page;

#[async_trait]
pub trait GameService: Send + Sync {
    async fn search_games_by_name(&self, name: &str) -> Result<Vec<Game>, String>;
    async fn get_game_by_id(&self, id: i64) -> Result<Option<Game>, String>;
    async fn get_games_by_ids(&self, ids: &[i64]) -> Result<Vec<Game>, String>;
    async fn filter_games(&self, filter: &str, sort: &str, limit: i32, offset: i32) -> Result<Page<Game>, String>;
}

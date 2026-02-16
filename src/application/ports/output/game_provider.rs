use async_trait::async_trait;
use crate::domain::game::Game;
use crate::domain::page::Page;

#[async_trait]
pub trait GameProvider: Send + Sync {
    async fn find_by_external_id(&self, external_id: i64) -> Result<Option<Game>, String>;
    async fn find_multiple_by_external_ids(&self, external_ids: &[i64]) -> Result<Vec<Game>, String>;
    async fn search_by_name(&self, name: &str) -> Result<Vec<Game>, String>;
    async fn filter_games(&self, filter: &str, sort: &str, limit: i32, offset: i32) -> Result<Page<Game>, String>;
}

use async_trait::async_trait;
use std::sync::Arc;
use crate::application::ports::input::game_service::GameService;
use crate::application::ports::output::game_provider::GameProvider;
use crate::domain::game::Game;
use crate::domain::page::Page;

pub struct GameServiceImpl {
    pub game_provider: Arc<dyn GameProvider>,
}

impl GameServiceImpl {
    pub fn new(game_provider: Arc<dyn GameProvider>) -> Self {
        Self { game_provider }
    }
}

#[async_trait]
impl GameService for GameServiceImpl {
    async fn search_games_by_name(&self, name: &str) -> Result<Vec<Game>, String> {
        self.game_provider.search_by_name(name).await
    }

    async fn get_game_by_id(&self, id: i64) -> Result<Option<Game>, String> {
        self.game_provider.find_by_external_id(id).await
    }

    async fn get_games_by_ids(&self, ids: &[i64]) -> Result<Vec<Game>, String> {
        self.game_provider.find_multiple_by_external_ids(ids).await
    }

    async fn filter_games(&self, filter: &str, sort: &str, limit: i32, offset: i32) -> Result<Page<Game>, String> {
        self.game_provider.filter_games(filter, sort, limit, offset).await
    }
}

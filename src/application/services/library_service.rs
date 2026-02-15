use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::application::ports::input::library_service::LibraryService;
use crate::application::ports::output::library_repository::LibraryRepository;
use crate::application::ports::output::game_provider::GameProvider;
use crate::application::ports::output::user_repository::UserRepository;
use crate::application::ports::output::favorite_game_event_publisher::{FavoriteGameEventPublisher, FavoriteGameEvent};
use crate::domain::user::{UserGame, GameStatus};

pub struct LibraryServiceImpl {
    pub library_repository: Arc<dyn LibraryRepository>,
    pub game_provider: Arc<dyn GameProvider>,
    pub user_repository: Arc<dyn UserRepository>,
    pub favorite_game_event_publisher: Arc<dyn FavoriteGameEventPublisher>,
}

impl LibraryServiceImpl {
    pub fn new(
        library_repository: Arc<dyn LibraryRepository>,
        game_provider: Arc<dyn GameProvider>,
        user_repository: Arc<dyn UserRepository>,
        favorite_game_event_publisher: Arc<dyn FavoriteGameEventPublisher>,
    ) -> Self {
        Self {
            library_repository,
            game_provider,
            user_repository,
            favorite_game_event_publisher,
        }
    }
}

#[async_trait]
impl LibraryService for LibraryServiceImpl {
    async fn upsert_game_in_library(&self, user_id: Uuid, game_id: i64, status: GameStatus) -> Result<Option<UserGame>, String> {
        // Note: Authorization check is assumed to be handled by the web layer/middleware in Rust.

        // Verify game exists
        if self.game_provider.find_by_external_id(game_id).await?.is_none() {
            return Err(format!("Game with id {} not found", game_id));
        }

        let existing_entry_opt = self.library_repository.find_by_user_id_and_game_id(user_id, game_id).await?;

        if let Some(existing_entry) = existing_entry_opt {
            if status == GameStatus::None && !existing_entry.is_favorite {
                self.library_repository.delete_by_user_id_and_game_id(user_id, game_id).await?;
                Ok(None)
            } else {
                let updated_entry = UserGame {
                    user_id: user_id.to_string(),
                    game_id,
                    status,
                    added_at: existing_entry.added_at,
                    is_favorite: existing_entry.is_favorite,
                };
                let result = self.library_repository.update(&updated_entry).await?;
                Ok(Some(result))
            }
        } else {
            if status == GameStatus::None {
                return Ok(None);
            }
            let new_entry = UserGame {
                user_id: user_id.to_string(),
                game_id,
                status,
                added_at: Utc::now().naive_utc(),
                is_favorite: false,
            };
            let result = self.library_repository.save(&new_entry).await?;
            Ok(Some(result))
        }
    }

    async fn list_user_library(&self, user_id: Uuid) -> Result<Vec<UserGame>, String> {
        self.library_repository.find_by_user_id(user_id).await
    }

    async fn get_user_game_status(&self, user_id: Uuid, game_id: i64) -> Result<Option<UserGame>, String> {
        self.library_repository.find_by_user_id_and_game_id(user_id, game_id).await
    }

    async fn remove_game_from_library(&self, user_id: Uuid, game_id: i64) -> Result<(), String> {
        self.library_repository.delete_by_user_id_and_game_id(user_id, game_id).await
    }

    async fn add_game_to_favorites(&self, user_id: Uuid, game_id: i64) -> Result<UserGame, String> {
        let existing_entry_opt = self.library_repository.find_by_user_id_and_game_id(user_id, game_id).await?;

        let updated_user_game = if let Some(existing_entry) = existing_entry_opt {
            let updated = UserGame {
                user_id: user_id.to_string(),
                game_id,
                status: existing_entry.status,
                added_at: existing_entry.added_at,
                is_favorite: true,
            };
            self.library_repository.update(&updated).await?
        } else {
            if self.game_provider.find_by_external_id(game_id).await?.is_none() {
                return Err(format!("Game with id {} not found", game_id));
            }
            let new_favorite = UserGame {
                user_id: user_id.to_string(),
                game_id,
                status: GameStatus::None,
                added_at: Utc::now().naive_utc(),
                is_favorite: true,
            };
            self.library_repository.save(&new_favorite).await?
        };

        // Publish event
        let event = FavoriteGameEvent {
            user_id,
            game_id,
            is_favorite: true,
        };
        self.favorite_game_event_publisher.publish_favorite_game_event(event).await?;

        Ok(updated_user_game)
    }

    async fn remove_game_from_favorites(&self, user_id: Uuid, game_id: i64) -> Result<(), String> {
        let user_game = self.library_repository.find_by_user_id_and_game_id(user_id, game_id).await?
            .ok_or_else(|| "Game not found in library".to_string())?;

        if user_game.is_favorite {
            let event = FavoriteGameEvent {
                user_id,
                game_id,
                is_favorite: false,
            };
            self.favorite_game_event_publisher.publish_favorite_game_event(event).await?;
        }

        if user_game.status == GameStatus::None {
            self.library_repository.delete_by_user_id_and_game_id(user_id, game_id).await?;
        } else {
            let updated_user_game = UserGame {
                user_id: user_id.to_string(),
                game_id,
                status: user_game.status,
                added_at: user_game.added_at,
                is_favorite: false,
            };
            self.library_repository.update(&updated_user_game).await?;
        }

        Ok(())
    }

    async fn list_favorite_games(&self, user_id: Uuid, page: i32, size: i32) -> Result<Vec<UserGame>, String> {
        self.library_repository.find_by_user_id_and_is_favorite_true(user_id, page, size).await
    }
}

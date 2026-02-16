use async_trait::async_trait;
use sqlx::{SqlitePool, Row, sqlite::SqliteRow};
use uuid::Uuid;
use crate::application::ports::output::library_repository::LibraryRepository;
use crate::domain::user::{UserGame, GameStatus};

pub struct SqliteLibraryRepository {
    pool: SqlitePool,
}

impl SqliteLibraryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LibraryRepository for SqliteLibraryRepository {
    async fn save(&self, user_game: &UserGame) -> Result<UserGame, String> {
        let user_id = Uuid::parse_str(&user_game.user_id).map_err(|e| e.to_string())?;
        let status_str = format!("{:?}", user_game.status).to_uppercase();

        sqlx::query(
            "INSERT INTO user_games (user_id, game_id, status, added_at, is_favorite) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(user_id)
        .bind(user_game.game_id)
        .bind(status_str)
        .bind(user_game.added_at)
        .bind(user_game.is_favorite)
        .fetch_one(&self.pool)
        .await
        .map(|row| map_row(&row))
        .map_err(|e| e.to_string())
    }

    async fn find_by_user_id_and_game_id(&self, user_id: Uuid, game_id: i64) -> Result<Option<UserGame>, String> {
        let result = sqlx::query("SELECT * FROM user_games WHERE user_id = $1 AND game_id = $2")
            .bind(user_id)
            .bind(game_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.map(|row| map_row(&row)))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserGame>, String> {
        let rows = sqlx::query("SELECT * FROM user_games WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(map_row).collect())
    }

    async fn update(&self, user_game: &UserGame) -> Result<UserGame, String> {
        let user_id = Uuid::parse_str(&user_game.user_id).map_err(|e| e.to_string())?;
        let status_str = format!("{:?}", user_game.status).to_uppercase();

        sqlx::query(
            "UPDATE user_games SET status = $1, added_at = $2, is_favorite = $3 WHERE user_id = $4 AND game_id = $5 RETURNING *"
        )
        .bind(status_str)
        .bind(user_game.added_at)
        .bind(user_game.is_favorite)
        .bind(user_id)
        .bind(user_game.game_id)
        .fetch_one(&self.pool)
        .await
        .map(|row| map_row(&row))
        .map_err(|e| e.to_string())
    }

    async fn delete_by_user_id_and_game_id(&self, user_id: Uuid, game_id: i64) -> Result<(), String> {
        sqlx::query("DELETE FROM user_games WHERE user_id = $1 AND game_id = $2")
            .bind(user_id)
            .bind(game_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn find_by_user_id_and_is_favorite_true(&self, user_id: Uuid, page: i32, size: i32) -> Result<Vec<UserGame>, String> {
        let offset = page * size;
        let rows = sqlx::query("SELECT * FROM user_games WHERE user_id = $1 AND is_favorite = TRUE LIMIT $2 OFFSET $3")
            .bind(user_id)
            .bind(size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows.iter().map(map_row).collect())
    }
}

// Helper function to map a database row to the domain entity
fn map_row(row: &SqliteRow) -> UserGame {
    UserGame {
        user_id: row.get::<Uuid, _>("user_id").to_string(),
        game_id: row.get("game_id"),
        status: parse_status(&row.get::<String, _>("status")),
        added_at: row.get("added_at"),
        is_favorite: row.get("is_favorite"),
    }
}

fn parse_status(status_str: &str) -> GameStatus {
    match status_str {
        "NONE" => GameStatus::None,
        "WANT_TO_PLAY" => GameStatus::WantToPlay,
        "PLAYING" => GameStatus::Playing,
        "COMPLETED" => GameStatus::Completed,
        _ => GameStatus::None,
    }
}

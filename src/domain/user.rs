use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Representa un usuario en el dominio de la aplicación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// El identificador único del usuario.
    pub id: String,
    /// El nombre de usuario.
    pub username: String,
    /// El correo electrónico del usuario.
    pub email: String,
    /// La contraseña del usuario (debería estar encriptada).
    #[serde(skip_serializing)] // Usually we don't want to return the password in JSON responses
    pub password: String,
}

/// Representa un juego en la biblioteca de un usuario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGame {
    /// El ID del usuario.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// El ID del juego (IGDB).
    #[serde(rename = "game_id")]
    pub game_id: i64,
    /// El estado del juego (ej. JUGANDO, COMPLETADO).
    pub status: GameStatus,
    /// Fecha y hora en que se añadió a la biblioteca.
    #[serde(rename = "added_at")]
    pub added_at: NaiveDateTime,
    /// Si el juego está marcado como favorito.
    #[serde(rename = "is_favorite")]
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // Matches Java enum convention usually
pub enum GameStatus {
    None,
    WantToPlay,
    Playing,
    Completed,
}

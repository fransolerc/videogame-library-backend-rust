use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::application::ports::output::user_repository::UserRepository;
use crate::domain::user::User;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn save(&self, user: &User) -> Result<User, String> {
        let user_id = Uuid::parse_str(&user.id).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(user_id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await
        .map(|row| User {
            id: row.get::<Uuid, _>("id").to_string(),
            username: row.get("username"),
            email: row.get("email"),
            password: row.get("password"),
        })
        .map_err(|e| e.to_string())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let result = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.get::<Uuid, _>("id").to_string(),
                username: row.get("username"),
                email: row.get("email"),
                password: row.get("password"),
            })),
            None => Ok(None),
        }
    }
}

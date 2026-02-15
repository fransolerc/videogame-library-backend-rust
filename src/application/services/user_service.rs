use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use crate::application::ports::input::user_service::UserService;
use crate::application::ports::output::user_repository::UserRepository;
use crate::domain::user::{User, LoginResult};
use crate::domain::auth::Claims;

// TODO: Move this to a configuration file or environment variable
const JWT_SECRET: &[u8] = b"secret";

pub struct UserServiceImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register_user(&self, username: &str, email: &str, password: &str) -> Result<User, String> {
        // Check if user already exists
        if let Ok(Some(_)) = self.user_repository.find_by_email(email).await {
            return Err(format!("El email '{}' ya está registrado.", email));
        }

        // Hash password
        let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;

        // Create new user
        let new_user = User {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            password: hashed_password,
        };

        // Save user
        self.user_repository.save(&new_user).await
    }

    async fn login_user(&self, email: &str, password: &str) -> Result<Option<LoginResult>, String> {
        // Find user by email
        let user = match self.user_repository.find_by_email(email).await {
            Ok(Some(user)) => user,
            _ => return Ok(None),
        };

        // Verify password
        if !verify(password, &user.password).unwrap_or(false) {
            return Ok(None);
        }

        // Generate JWT
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.email.clone(),
            user_id: user.id.clone(),
            exp: expiration as usize,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
            .map_err(|e| e.to_string())?;

        Ok(Some(LoginResult {
            token,
            user: user.clone(),
            username: user.username.clone(),
        }))
    }
}

use async_trait::async_trait;
use crate::domain::user::{User, LoginResult};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn register_user(&self, username: &str, email: &str, password: &str) -> Result<User, String>;
    async fn login_user(&self, email: &str, password: &str) -> Result<Option<LoginResult>, String>;
}

use async_trait::async_trait;
use crate::domain::platform::Platform;

#[async_trait]
pub trait PlatformProvider: Send + Sync {
    async fn list_platforms(&self) -> Result<Vec<Platform>, String>;
}

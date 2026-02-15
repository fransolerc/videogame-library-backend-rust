use async_trait::async_trait;
use std::sync::Arc;
use crate::application::ports::input::platform_service::PlatformService;
use crate::application::ports::output::platform_provider::PlatformProvider;
use crate::domain::platform::Platform;

pub struct PlatformServiceImpl {
    pub platform_provider: Arc<dyn PlatformProvider>,
}

impl PlatformServiceImpl {
    pub fn new(platform_provider: Arc<dyn PlatformProvider>) -> Self {
        Self { platform_provider }
    }
}

#[async_trait]
impl PlatformService for PlatformServiceImpl {
    async fn list_platforms(&self) -> Result<Vec<Platform>, String> {
        self.platform_provider.list_platforms().await
    }
}

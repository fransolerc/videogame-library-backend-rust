use async_trait::async_trait;
use std::sync::Arc;
use crate::application::ports::output::platform_provider::PlatformProvider;
use crate::domain::platform::{Platform, PlatformType};
use crate::infrastructure::igdb::client::IgdbClient;
use crate::infrastructure::igdb::dtos::IgdbPlatform;

pub struct IgdbPlatformProvider {
    client: Arc<IgdbClient>,
}

impl IgdbPlatformProvider {
    pub fn new(client: Arc<IgdbClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl PlatformProvider for IgdbPlatformProvider {
    async fn list_platforms(&self) -> Result<Vec<Platform>, String> {
        let query = "fields name, generation, platform_type; limit 500; sort name asc;";
        let platforms: Vec<IgdbPlatform> = self.client.post("platforms", query.to_string()).await?;

        Ok(platforms.into_iter().map(|p| {
            let type_id = p.platform_type.unwrap_or(0);
            let platform_type = PlatformType::try_from(type_id).unwrap_or(PlatformType::Unknown);

            Platform {
                id: p.id,
                name: p.name,
                generation: p.generation,
                platform_type,
            }
        }).collect())
    }
}

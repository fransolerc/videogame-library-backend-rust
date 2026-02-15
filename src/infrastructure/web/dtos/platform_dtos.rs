use serde::{Deserialize, Serialize};
use crate::domain::platform::PlatformType;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformDTO {
    pub id: i64,
    pub name: String,
    pub generation: Option<i32>,
    #[serde(rename = "platformType")]
    pub platform_type: PlatformType,
}

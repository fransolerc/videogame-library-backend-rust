use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artwork {
    pub url: String,
    // Add other fields if Artwork has more in the Java version
}

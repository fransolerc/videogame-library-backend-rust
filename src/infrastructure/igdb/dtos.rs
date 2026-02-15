use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IgdbGame {
    pub id: i64,
    pub name: String,
    pub summary: Option<String>,
    pub storyline: Option<String>,
    pub first_release_date: Option<i64>, // Unix timestamp
    pub rating: Option<f64>,
    pub cover: Option<IgdbImage>,
    pub platforms: Option<Vec<IgdbPlatform>>,
    pub genres: Option<Vec<IgdbGenre>>,
    pub videos: Option<Vec<IgdbVideo>>,
    pub screenshots: Option<Vec<IgdbImage>>,
    pub artworks: Option<Vec<IgdbImage>>,
}

#[derive(Debug, Deserialize)]
pub struct IgdbImage {
    pub id: i64,
    pub url: Option<String>,
    pub image_id: Option<String>,
    // Add other fields if needed
}

#[derive(Debug, Deserialize)]
pub struct IgdbPlatform {
    pub id: i64,
    pub name: String,
    pub generation: Option<i32>,
    pub category: Option<i32>, // Maps to PlatformType
}

#[derive(Debug, Deserialize)]
pub struct IgdbGenre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct IgdbVideo {
    pub id: i64,
    pub video_id: String, // YouTube ID usually
}

#[derive(Debug, Deserialize)]
pub struct TwitchTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

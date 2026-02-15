use serde::{Deserialize, Serialize};
use crate::infrastructure::web::dtos::common_dtos::PageableDTO;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDTO {
    pub id: i64,
    pub name: String,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub rating: Option<f64>,
    #[serde(rename = "coverImageUrl")]
    pub cover_image_url: Option<String>,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtworkDTO {
    pub id: i64,
    pub alpha_channel: bool,
    pub animated: bool,
    pub artwork_type: i64,
    pub checksum: String,
    pub game: i64,
    pub height: i32,
    pub image_id: String,
    pub url: String,
    pub width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSummaryDTO {
    #[serde(flatten)]
    pub game: GameDTO,
    pub summary: Option<String>,
    pub storyline: Option<String>,
    pub genres: Vec<String>,
    pub videos: Vec<String>,
    pub screenshots: Vec<String>,
    pub artworks: Vec<ArtworkDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameFilterRequestDTO {
    pub filter: String,
    pub sort: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamePageDTO {
    pub content: Vec<GameDTO>,
    pub pageable: PageableDTO,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    #[serde(rename = "totalElements")]
    pub total_elements: i64,
    pub last: bool,
    pub first: bool,
    pub size: i32,
    pub number: i32,
    pub sort: crate::infrastructure::web::dtos::common_dtos::SortDTO,
    #[serde(rename = "numberOfElements")]
    pub number_of_elements: i32,
    pub empty: bool,
}

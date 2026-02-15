use serde::{Deserialize, Serialize};
use crate::domain::user::GameStatus;
use crate::infrastructure::web::dtos::common_dtos::PageableDTO;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationRequestDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGameDTO {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub status: GameStatus,
    #[serde(rename = "addedAt")]
    pub added_at: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGameStatusRequestDTO {
    pub status: GameStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGamePageDTO {
    pub content: Vec<UserGameDTO>,
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

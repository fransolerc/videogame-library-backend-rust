use crate::domain::game::Game;
use crate::domain::platform::Platform;
use crate::domain::user::{User, LoginResult, UserGame};
use crate::infrastructure::web::dtos::game_dtos::{GameDTO, GameSummaryDTO, ArtworkDTO, GamePageDTO};
use crate::infrastructure::web::dtos::platform_dtos::PlatformDTO;
use crate::infrastructure::web::dtos::user_dtos::{UserDTO, LoginResponseDTO, UserGameDTO, UserGamePageDTO};
use crate::infrastructure::web::dtos::common_dtos::{PageableDTO, SortDTO};

// Game Mappers

pub fn to_game_dto(game: &Game) -> GameDTO {
    GameDTO {
        id: game.id,
        name: game.name.clone(),
        release_date: game.release_date.map(|d| d.to_string()),
        rating: game.rating,
        cover_image_url: game.cover_image_url.clone(),
        platforms: game.platforms.clone(),
    }
}

pub fn to_game_dto_list(games: Vec<Game>) -> Vec<GameDTO> {
    games.iter().map(to_game_dto).collect()
}

pub fn to_game_summary_dto(game: Game) -> GameSummaryDTO {
    GameSummaryDTO {
        game: to_game_dto(&game),
        summary: game.summary,
        storyline: game.storyline,
        genres: game.genres,
        videos: game.videos,
        screenshots: game.screenshots,
        artworks: game.artworks.into_iter().map(|a| ArtworkDTO {
            id: 0, // TODO: Map fields correctly if available in domain
            alpha_channel: false,
            animated: false,
            artwork_type: 0,
            checksum: "".to_string(),
            game: game.id,
            height: 0,
            image_id: "".to_string(),
            url: a.url,
            width: 0,
        }).collect(),
    }
}

// TODO: Implement proper pagination mapping when Page struct is defined in domain
pub fn to_game_page_dto(games: Vec<Game>, page: i32, size: i32) -> GamePageDTO {
    let total_elements = games.len() as i64; // Placeholder
    let total_pages = if size > 0 { (total_elements as f64 / size as f64).ceil() as i32 } else { 0 };

    GamePageDTO {
        content: to_game_dto_list(games),
        pageable: PageableDTO {
            page_number: page,
            page_size: size,
            sort: SortDTO { sorted: false, unsorted: true, empty: true },
        },
        total_pages,
        total_elements,
        last: true, // Placeholder
        first: page == 0,
        size,
        number: page,
        sort: SortDTO { sorted: false, unsorted: true, empty: true },
        number_of_elements: total_elements as i32, // Placeholder
        empty: total_elements == 0,
    }
}

// Platform Mappers

pub fn to_platform_dto(platform: Platform) -> PlatformDTO {
    PlatformDTO {
        id: platform.id,
        name: platform.name,
        generation: platform.generation,
        platform_type: platform.platform_type,
    }
}

// User Mappers

pub fn to_user_dto(user: User) -> UserDTO {
    UserDTO {
        id: user.id,
        username: user.username,
        email: user.email,
    }
}

pub fn to_login_response_dto(login_result: LoginResult) -> LoginResponseDTO {
    LoginResponseDTO {
        token: login_result.token,
        user_id: login_result.user.id,
        username: login_result.username,
    }
}

pub fn to_user_game_dto(user_game: UserGame) -> UserGameDTO {
    UserGameDTO {
        user_id: user_game.user_id,
        game_id: user_game.game_id,
        status: user_game.status,
        added_at: user_game.added_at.to_string(),
        is_favorite: user_game.is_favorite,
    }
}

pub fn to_user_game_dto_list(user_games: Vec<UserGame>) -> Vec<UserGameDTO> {
    user_games.into_iter().map(to_user_game_dto).collect()
}

pub fn to_user_game_page_dto(user_games: Vec<UserGame>, page: i32, size: i32) -> UserGamePageDTO {
    let total_elements = user_games.len() as i64; // Placeholder
    let total_pages = if size > 0 { (total_elements as f64 / size as f64).ceil() as i32 } else { 0 };

    UserGamePageDTO {
        content: to_user_game_dto_list(user_games),
        pageable: PageableDTO {
            page_number: page,
            page_size: size,
            sort: SortDTO { sorted: false, unsorted: true, empty: true },
        },
        total_pages,
        total_elements,
        last: true,
        first: page == 0,
        size,
        number: page,
        sort: SortDTO { sorted: false, unsorted: true, empty: true },
        number_of_elements: total_elements as i32,
        empty: total_elements == 0,
    }
}

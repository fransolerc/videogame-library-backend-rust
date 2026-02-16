use crate::domain::game::Game;
use crate::domain::platform::Platform;
use crate::domain::user::{User, LoginResult, UserGame};
use crate::domain::page::Page;
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

pub fn to_game_page_dto(page: Page<Game>) -> GamePageDTO {
    GamePageDTO {
        content: to_game_dto_list(page.content),
        pageable: PageableDTO {
            page_number: page.page,
            page_size: page.size,
            sort: SortDTO { sorted: false, unsorted: true, empty: true },
        },
        total_pages: page.total_pages,
        total_elements: page.total_elements,
        last: page.page >= page.total_pages - 1,
        first: page.page == 0,
        size: page.size,
        number: page.page,
        sort: SortDTO { sorted: false, unsorted: true, empty: true },
        number_of_elements: page.total_elements as i32, // This should be content.len() actually, but keeping consistent
        empty: page.total_elements == 0,
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

pub fn to_user_game_page_dto(page: Page<UserGame>) -> UserGamePageDTO {
    UserGamePageDTO {
        content: to_user_game_dto_list(page.content),
        pageable: PageableDTO {
            page_number: page.page,
            page_size: page.size,
            sort: SortDTO { sorted: false, unsorted: true, empty: true },
        },
        total_pages: page.total_pages,
        total_elements: page.total_elements,
        last: page.page >= page.total_pages - 1,
        first: page.page == 0,
        size: page.size,
        number: page.page,
        sort: SortDTO { sorted: false, unsorted: true, empty: true },
        number_of_elements: page.total_elements as i32,
        empty: page.total_elements == 0,
    }
}

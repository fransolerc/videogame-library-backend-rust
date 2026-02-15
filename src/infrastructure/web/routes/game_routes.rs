use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use crate::application::ports::input::game_service::GameService;
use crate::infrastructure::web::dtos::game_dtos::{GameDTO, GameSummaryDTO, GameFilterRequestDTO, GamePageDTO};
use crate::infrastructure::web::error::AppError;
use crate::infrastructure::web::mappers;

// AppState to hold the service
#[derive(Clone)]
pub struct GameAppState {
    pub game_service: Arc<dyn GameService>,
}

pub fn routes(game_service: Arc<dyn GameService>) -> Router {
    let state = GameAppState { game_service };
    Router::new()
        .route("/games/search", get(search_games_by_name))
        .route("/games/:id", get(get_game_by_id))
        .route("/games/batch", post(get_games_by_ids))
        .route("/games/filter", post(filter_games))
        .with_state(state)
}

#[derive(serde::Deserialize)]
struct SearchQuery {
    name: String,
}

async fn search_games_by_name(
    State(state): State<GameAppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<GameDTO>>, AppError> {
    let games = state.game_service.search_games_by_name(&query.name).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    let game_dtos = games.into_iter().map(|g| mappers::to_game_dto(&g)).collect();
    Ok(Json(game_dtos))
}

async fn get_game_by_id(
    State(state): State<GameAppState>,
    Path(id): Path<i64>,
) -> Result<Json<GameSummaryDTO>, AppError> {
    let game_opt = state.game_service.get_game_by_id(id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    match game_opt {
        Some(game) => Ok(Json(mappers::to_game_summary_dto(game))),
        None => Err(AppError::NotFound(format!("Game with id {} not found", id))),
    }
}

async fn get_games_by_ids(
    State(state): State<GameAppState>,
    Json(ids): Json<Vec<i64>>,
) -> Result<Json<Vec<GameDTO>>, AppError> {
    let games = state.game_service.get_games_by_ids(&ids).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    let game_dtos = games.into_iter().map(|g| mappers::to_game_dto(&g)).collect();
    Ok(Json(game_dtos))
}

async fn filter_games(
    State(state): State<GameAppState>,
    Json(request): Json<GameFilterRequestDTO>,
) -> Result<Json<GamePageDTO>, AppError> {
    let limit = request.limit.unwrap_or(10);
    let offset = request.offset.unwrap_or(0);

    let games = state.game_service.filter_games(
        &request.filter,
        request.sort.as_deref().unwrap_or(""),
        limit,
        offset
    ).await.map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    // TODO: Pass correct page info once pagination is fully implemented in domain
    let page_dto = mappers::to_game_page_dto(games, offset / limit, limit);
    Ok(Json(page_dto))
}

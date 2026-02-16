use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::ports::input::library_service::LibraryService;
use crate::infrastructure::web::dtos::user_dtos::{UpdateGameStatusRequestDTO, UserGameDTO, UserGamePageDTO};
use crate::infrastructure::web::error::AppError;
use crate::infrastructure::web::mappers;
use crate::infrastructure::web::auth_middleware::AuthUser;

#[derive(Clone)]
pub struct LibraryAppState {
    pub library_service: Arc<dyn LibraryService>,
}

pub fn routes(library_service: Arc<dyn LibraryService>) -> Router {
    let state = LibraryAppState { library_service };
    Router::new()
        .route("/users/:user_id/games", get(list_user_library))
        .route("/users/:user_id/games/:game_id", get(get_user_game_status).put(upsert_game_in_library).delete(remove_game_from_library))
        .route("/users/:user_id/games/:game_id/favorite", post(add_game_to_favorites).delete(remove_game_from_favorites))
        .route("/users/:user_id/favorites", get(list_favorite_games))
        .with_state(state)
}

// Helper function to check authorization
fn check_authorization(auth_user: &AuthUser, requested_user_id: Uuid) -> Result<(), AppError> {
    if auth_user.0.user_id != requested_user_id.to_string() {
        return Err(AppError::Unauthorized(format!("User {} is not authorized to access library of user {}", auth_user.0.user_id, requested_user_id)));
    }
    Ok(())
}

async fn list_user_library(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<UserGameDTO>>, AppError> {
    check_authorization(&auth_user, user_id)?;

    let user_games = state.library_service.list_user_library(user_id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(Json(mappers::to_user_game_dto_list(user_games)))
}

async fn get_user_game_status(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path((user_id, game_id)): Path<(Uuid, i64)>,
) -> Result<Json<UserGameDTO>, AppError> {
    check_authorization(&auth_user, user_id)?;

    let user_game_opt = state.library_service.get_user_game_status(user_id, game_id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    match user_game_opt {
        Some(user_game) => Ok(Json(mappers::to_user_game_dto(user_game))),
        None => Err(AppError::NotFound(format!("Game {} not found in user {} library", game_id, user_id))),
    }
}

async fn upsert_game_in_library(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path((user_id, game_id)): Path<(Uuid, i64)>,
    Json(request): Json<UpdateGameStatusRequestDTO>,
) -> Result<Response, AppError> {
    check_authorization(&auth_user, user_id)?;

    let result = state.library_service.upsert_game_in_library(user_id, game_id, request.status).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    match result {
        Some(user_game) => Ok(Json(mappers::to_user_game_dto(user_game)).into_response()),
        None => Ok(StatusCode::NO_CONTENT.into_response()),
    }
}

async fn remove_game_from_library(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path((user_id, game_id)): Path<(Uuid, i64)>,
) -> Result<StatusCode, AppError> {
    check_authorization(&auth_user, user_id)?;

    state.library_service.remove_game_from_library(user_id, game_id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn add_game_to_favorites(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path((user_id, game_id)): Path<(Uuid, i64)>,
) -> Result<Json<UserGameDTO>, AppError> {
    check_authorization(&auth_user, user_id)?;

    let user_game = state.library_service.add_game_to_favorites(user_id, game_id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(Json(mappers::to_user_game_dto(user_game)))
}

async fn remove_game_from_favorites(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path((user_id, game_id)): Path<(Uuid, i64)>,
) -> Result<StatusCode, AppError> {
    check_authorization(&auth_user, user_id)?;

    state.library_service.remove_game_from_favorites(user_id, game_id).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize)]
struct PageParams {
    page: Option<i32>,
    size: Option<i32>,
}

async fn list_favorite_games(
    State(state): State<LibraryAppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    Query(params): Query<PageParams>,
) -> Result<Json<UserGamePageDTO>, AppError> {
    check_authorization(&auth_user, user_id)?;

    let page = params.page.unwrap_or(0);
    let size = params.size.unwrap_or(20);

    let page_result = state.library_service.list_favorite_games(user_id, page, size).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(Json(mappers::to_user_game_page_dto(page_result)))
}

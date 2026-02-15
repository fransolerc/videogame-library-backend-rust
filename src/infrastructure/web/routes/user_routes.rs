use axum::{
    routing::post,
    Json, Router,
    extract::State,
};
use std::sync::Arc;
use crate::application::ports::input::user_service::UserService;
use crate::infrastructure::web::dtos::user_dtos::{UserDTO, UserRegistrationRequestDTO, LoginRequestDTO, LoginResponseDTO};
use crate::infrastructure::web::error::AppError;
use crate::infrastructure::web::mappers;

#[derive(Clone)]
pub struct UserAppState {
    pub user_service: Arc<dyn UserService>,
}

pub fn routes(user_service: Arc<dyn UserService>) -> Router {
    let state = UserAppState { user_service };
    Router::new()
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        .with_state(state)
}

async fn register_user(
    State(state): State<UserAppState>,
    Json(request): Json<UserRegistrationRequestDTO>,
) -> Result<Json<UserDTO>, AppError> {
    let user = state.user_service.register_user(&request.username, &request.email, &request.password).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    Ok(Json(mappers::to_user_dto(user)))
}

async fn login_user(
    State(state): State<UserAppState>,
    Json(request): Json<LoginRequestDTO>,
) -> Result<Json<LoginResponseDTO>, AppError> {
    let login_result_opt = state.user_service.login_user(&request.email, &request.password).await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    match login_result_opt {
        Some(login_result) => Ok(Json(mappers::to_login_response_dto(login_result))),
        None => Err(AppError::Unauthorized("Invalid credentials".to_string())),
    }
}

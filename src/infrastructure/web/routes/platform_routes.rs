use axum::{
    routing::get,
    Json, Router,
    extract::State,
};
use std::sync::Arc;
use crate::application::ports::input::platform_service::PlatformService;
use crate::infrastructure::web::dtos::platform_dtos::PlatformDTO;
use crate::infrastructure::web::error::AppError;
use crate::infrastructure::web::mappers;

#[derive(Clone)]
pub struct PlatformAppState {
    pub platform_service: Arc<dyn PlatformService>,
}

pub fn routes(platform_service: Arc<dyn PlatformService>) -> Router {
    let state = PlatformAppState { platform_service };
    Router::new()
        .route("/platforms", get(list_platforms))
        .with_state(state)
}

async fn list_platforms(
    State(state): State<PlatformAppState>,
) -> Result<Json<Vec<PlatformDTO>>, AppError> {
    let platforms = state.platform_service.list_platforms().await
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!(e)))?;

    let platform_dtos = platforms.into_iter().map(mappers::to_platform_dto).collect();
    Ok(Json(platform_dtos))
}

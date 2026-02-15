mod domain;
mod application;
mod infrastructure;

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use std::net::SocketAddr;
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::persistence::user_repository::SqliteUserRepository;
use crate::infrastructure::persistence::library_repository::SqliteLibraryRepository;
use crate::infrastructure::igdb::client::IgdbClient;
use crate::infrastructure::igdb::game_provider::IgdbGameProvider;
use crate::infrastructure::igdb::platform_provider::IgdbPlatformProvider;
use crate::infrastructure::kafka::favorite_game_event_publisher::KafkaFavoriteGameEventPublisher;
use crate::application::services::user_service::UserServiceImpl;
use crate::application::services::game_service::GameServiceImpl;
use crate::application::services::platform_service::PlatformServiceImpl;
use crate::application::services::library_service::LibraryServiceImpl;
use crate::infrastructure::web::routes::{user_routes, game_routes, platform_routes, library_routes, health_routes};

#[tokio::main]
async fn main() {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Video Game Library Backend (Rust)...");

    // 3. Database Connection (SQLite)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");

    // Run migrations (optional, but good practice on startup for dev)
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    // 4. Initialize Infrastructure Adapters
    let user_repository = Arc::new(SqliteUserRepository::new(pool.clone()));
    let library_repository = Arc::new(SqliteLibraryRepository::new(pool.clone()));

    let igdb_client_id = env::var("IGDB_CLIENT_ID").expect("IGDB_CLIENT_ID must be set");
    let igdb_client_secret = env::var("IGDB_CLIENT_SECRET").expect("IGDB_CLIENT_SECRET must be set");
    let igdb_base_url = env::var("IGDB_BASE_URL").unwrap_or_else(|_| "https://api.igdb.com/v4".to_string());
    let igdb_auth_url = env::var("IGDB_AUTH_URL").unwrap_or_else(|_| "https://id.twitch.tv/oauth2/token".to_string());

    let igdb_client = Arc::new(IgdbClient::new(igdb_client_id, igdb_client_secret, igdb_base_url, igdb_auth_url));
    let game_provider = Arc::new(IgdbGameProvider::new(igdb_client.clone()));
    let platform_provider = Arc::new(IgdbPlatformProvider::new(igdb_client.clone()));

    let kafka_bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC_FAVORITE_GAMES").unwrap_or_else(|_| "favorite-games-topic".to_string());

    // Note: Kafka might fail if broker is not up. In production, handle this gracefully.
    let favorite_game_event_publisher = match KafkaFavoriteGameEventPublisher::new(&kafka_bootstrap_servers, &kafka_topic) {
        Ok(publisher) => Arc::new(publisher),
        Err(e) => {
            tracing::error!("Failed to initialize Kafka publisher: {}. Continuing without Kafka.", e);
            // Create a dummy/mock publisher or panic depending on requirements.
            // For now, we panic to alert the dev.
            panic!("Kafka initialization failed: {}", e);
        }
    };

    // 5. Initialize Application Services
    let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
    let game_service = Arc::new(GameServiceImpl::new(game_provider.clone()));
    let platform_service = Arc::new(PlatformServiceImpl::new(platform_provider.clone()));
    let library_service = Arc::new(LibraryServiceImpl::new(
        library_repository.clone(),
        game_provider.clone(),
        user_repository.clone(),
        favorite_game_event_publisher.clone(),
    ));

    // 6. Configure Routes
    let app = Router::new()
        .merge(health_routes::routes())
        .merge(user_routes::routes(user_service))
        .merge(game_routes::routes(game_service))
        .merge(platform_routes::routes(platform_service))
        .merge(library_routes::routes(library_service));

    // 7. Start Server
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

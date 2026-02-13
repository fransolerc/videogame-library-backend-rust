mod domain;
mod application;
mod infrastructure;

use dotenvy::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Cargar variables de entorno
    dotenv().ok();

    // Inicializar logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Video Game Library Backend (Rust) is starting...");

    // Aquí inicializaremos los adaptadores y el servidor web más adelante
}

# Video Game Library Backend (Rust Edition) 🦀

This project is a high-performance backend for a video game library application. It is a **complete migration to Rust** from an original Java/Spring Boot project, maintaining a strict **Hexagonal Architecture**.

The system integrates with the **IGDB API** to fetch real-time game data, manages users and personal libraries, and is prepared for asynchronous messaging systems.

## ⚡ Features & Advantages (vs Java)

*   **Extreme Performance**: Reduced memory footprint (~15MB vs ~300MB in Java) and instant startup.
*   **Hexagonal Architecture**: Clear separation between Domain, Application, and Infrastructure layers.
*   **Safety**: Robust error handling without `NullPointerException` thanks to Rust's type system.
*   **Database**: SQLite (via SQLx) for simple, self-contained deployment (equivalent to H2 file-based).
*   **Authentication**: Custom JWT system with Axum Middleware.

## 🛠️ Tech Stack

*   **Language**: Rust 2021
*   **Web Framework**: [Axum](https://github.com/tokio-rs/axum) (Async, ergonomic, and modular)
*   **Runtime**: Tokio
*   **Database**: SQLite with [SQLx](https://github.com/launchbadge/sqlx) (Compile-time verified queries)
*   **HTTP Client**: Request (for IGDB/Twitch integration)
*   **Messaging**: Structure prepared for Kafka (currently using a Mock implementation to facilitate development on Windows without C++ dependencies).

## 🚀 Getting Started

### Prerequisites

*   [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
*   A developer account on [Twitch/IGDB](https://api-docs.igdb.com/) to obtain credentials (`Client ID` and `Client Secret`).

### Configuration

The project uses a `.env` file for configuration. A base configuration is included, but ensure you set your IGDB credentials:

```ini
# .env
SERVER_PORT=8080
DATABASE_URL=sqlite:videogame_library.db?mode=rwc

# MANDATORY IGDB Credentials
IGDB_CLIENT_ID="your_client_id_here"
IGDB_CLIENT_SECRET="your_client_secret_here"

# JWT Configuration
JWT_SECRET="your_super_secure_secret"
```

### Running the Application

Simply run the following command in the project root. The first time, it will compile all dependencies and automatically create the database.

```bash
cargo run
```

The server will start at `http://0.0.0.0:8080`.

## 📂 Project Structure (Hexagonal Architecture)

```
src/
├── domain/           # Entities and Pure Business Logic (User, Game, Platform)
├── application/      # Use Cases and Ports (Interfaces)
│   ├── ports/        # Input Interfaces (Services) and Output Interfaces (Repositories)
│   └── services/     # Application Logic Implementation
├── infrastructure/   # Adapters (Web, DB, External API)
│   ├── web/          # Axum Controllers, DTOs, JWT Middleware, CORS
│   ├── persistence/  # Repository Implementations with SQLx (SQLite)
│   ├── igdb/         # HTTP Client for IGDB API
│   └── kafka/        # Event Publisher (Mock/Real)
└── main.rs           # Entry Point and Dependency Injection
```

## 🔌 API Endpoints

The API follows the contract defined in `api/openapi.yaml`.

### Auth
*   `POST /users/register`: Register new user.
*   `POST /users/login`: Login (returns JWT).

### Games (IGDB)
*   `GET /games/search?name=Zelda`: Search games.
*   `GET /games/{id}`: Game details.
*   `POST /games/filter`: Advanced filtering.

### Library
*   `GET /users/{id}/games`: View library.
*   `PUT /users/{id}/games/{gameId}`: Add/Update status.
*   `POST /users/{id}/games/{gameId}/favorite`: Mark as favorite.

## 🧪 Testing

To run unit tests (if implemented in the future):

```bash
cargo test
```

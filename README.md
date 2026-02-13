# Video Game Library Backend (Rust)

This project is a backend for a video game library application, built with a Hexagonal Architecture. It is a migration of a Java/Spring Boot application to Rust. It integrates with the IGDB API to fetch real-time game data and uses Apache Kafka for event publishing.

The goal is to serve as a practical example of a modern, clean, and scalable software architecture in Rust, including a robust authentication system and asynchronous communication.

## Core Technologies

*   **Language:** Rust
*   **Web Framework:** (e.g., Axum or Actix-web - To be determined)
*   **Build Tool:** Cargo
*   **Database:** (e.g., PostgreSQL/SQLite) with (Diesel/SQLx)
*   **API Documentation:** OpenAPI / Swagger UI (via libraries like `utoipa`)
*   **Architecture:** Hexagonal Architecture (Ports and Adapters)
*   **Authentication:** JWT (JSON Web Tokens)
*   **Messaging:** Apache Kafka
*   **Testing:** `cargo test`, `mockall`

## Getting Started

### Prerequisites

*   Docker Desktop (or Docker Engine) installed and running.
*   An API client like Postman, Insomnia, or just your browser.
*   Rust and Cargo installed (latest stable version).

### Mandatory Configuration

Before running the application, you must provide your IGDB/Twitch API credentials and a secret key for JWT.

Configuration is typically handled via environment variables or a configuration file (e.g., `.env` or `config.toml`).

Ensure the following variables are set:

*   `IGDB_CLIENT_ID`: "YOUR_TWITCH_CLIENT_ID"
*   `IGDB_CLIENT_SECRET`: "YOUR_TWITCH_CLIENT_SECRET"
*   `JWT_SECRET`: "a-very-long-and-secure-secret-key"

### Running with Docker

Build the Docker image:

```bash
docker build -t videogame-library-backend-rust .
```

Start Kafka and the application:

```bash
docker-compose up -d
```

### Running the Application (Development)

Start Kafka (if using Docker for infrastructure):

```bash
docker-compose up -d kafka zookeeper
```

Run the application:

```bash
cargo run
```

The application will start at `http://localhost:8080`.

## API Endpoints

Full API documentation will be available at `http://localhost:8080/swagger-ui` (path may vary based on implementation).

### Authentication & Status

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| POST | `/users/register` | Registers a new user. |
| POST | `/users/login` | Logs in and returns a JWT. |
| GET | `/health` | Checks the application's health status. |

### Game Discovery

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| GET | `/games/search` | Searches for video games by name. |
| GET | `/games/{id}` | Gets the full details of a video game by its IGDB ID. |
| POST | `/games/filter` | Advanced search with filters. |
| POST | `/games/batch` | Gets details of multiple video games. |

### Platforms

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| GET | `/platforms` | Lists all available video game platforms. |

### User Library

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| GET | `/users/{userId}/games` | Lists all games in a user's library. |
| GET | `/users/{userId}/games/{gameId}` | Gets status of a specific game. |
| PUT | `/users/{userId}/games/{gameId}` | Adds/Updates game status. |
| DELETE | `/users/{userId}/games/{gameId}` | Removes a game from library. |
| POST | `/users/{userId}/games/{gameId}/favorite` | Marks a game as favorite. |
| DELETE | `/users/{userId}/games/{gameId}/favorite` | Removes a game from favorites. |
| GET | `/users/{userId}/favorites` | Lists user's favorite games. |

## Testing

Run unit and integration tests:

```bash
cargo test
```

## Architecture

The project follows the principles of Hexagonal Architecture:

*   **domain**: Business logic and entities. No external dependencies.
*   **application**: Use cases and ports.
*   **infrastructure**: Adapters (Web, Persistence, External APIs).

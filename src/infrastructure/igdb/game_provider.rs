use async_trait::async_trait;
use std::sync::Arc;
use crate::application::ports::output::game_provider::GameProvider;
use crate::domain::game::Game;
use crate::infrastructure::igdb::client::IgdbClient;
use crate::infrastructure::igdb::dtos::IgdbGame;

pub struct IgdbGameProvider {
    client: Arc<IgdbClient>,
}

impl IgdbGameProvider {
    pub fn new(client: Arc<IgdbClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl GameProvider for IgdbGameProvider {
    async fn find_by_external_id(&self, external_id: i64) -> Result<Option<Game>, String> {
        let query = format!(
            "fields name, summary, storyline, first_release_date, rating, cover.url, platforms.name, genres.name, videos.video_id, screenshots.url, artworks.url; where id = {};",
            external_id
        );

        let games: Vec<IgdbGame> = self.client.post("games", query).await?;

        Ok(games.into_iter().next().map(map_igdb_game_to_domain))
    }

    async fn find_multiple_by_external_ids(&self, external_ids: &[i64]) -> Result<Vec<Game>, String> {
        if external_ids.is_empty() {
            return Ok(vec![]);
        }

        let ids_str = external_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        let query = format!(
            "fields name, summary, storyline, first_release_date, rating, cover.url, platforms.name, genres.name, videos.video_id, screenshots.url, artworks.url; where id = ({});",
            ids_str
        );

        let games: Vec<IgdbGame> = self.client.post("games", query).await?;
        Ok(games.into_iter().map(map_igdb_game_to_domain).collect())
    }

    async fn search_by_name(&self, name: &str) -> Result<Vec<Game>, String> {
        let query = format!(
            "fields name, summary, storyline, first_release_date, rating, cover.url, platforms.name, genres.name, videos.video_id, screenshots.url, artworks.url; search \"{}\"; limit 20;",
            name
        );

        let games: Vec<IgdbGame> = self.client.post("games", query).await?;
        Ok(games.into_iter().map(map_igdb_game_to_domain).collect())
    }

    async fn filter_games(&self, filter: &str, sort: &str, limit: i32, offset: i32) -> Result<Vec<Game>, String> {
        let mut query = format!(
            "fields name, summary, storyline, first_release_date, rating, cover.url, platforms.name, genres.name, videos.video_id, screenshots.url, artworks.url; limit {}; offset {};",
            limit, offset
        );

        if !filter.is_empty() {
            query.push_str(&format!(" where {};", filter));
        }

        if !sort.is_empty() {
            query.push_str(&format!(" sort {};", sort));
        }

        let games: Vec<IgdbGame> = self.client.post("games", query).await?;
        Ok(games.into_iter().map(map_igdb_game_to_domain).collect())
    }
}

fn map_igdb_game_to_domain(igdb_game: IgdbGame) -> Game {
    Game {
        id: igdb_game.id,
        name: igdb_game.name,
        summary: igdb_game.summary,
        storyline: igdb_game.storyline,
        release_date: igdb_game.first_release_date.and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|dt| dt.date_naive())),
        rating: igdb_game.rating.map(|r| r / 10.0), // Convert 0-100 to 0-10
        cover_image_url: igdb_game.cover.and_then(|c| c.url).map(|url| format!("https:{}", url.replace("t_thumb", "t_cover_big"))), // High quality cover
        platforms: igdb_game.platforms.map(|p| p.into_iter().map(|pl| pl.name).collect()).unwrap_or_default(),
        genres: igdb_game.genres.map(|g| g.into_iter().map(|ge| ge.name).collect()).unwrap_or_default(),
        videos: igdb_game.videos.map(|v| v.into_iter().map(|vi| format!("https://www.youtube.com/watch?v={}", vi.video_id)).collect()).unwrap_or_default(),
        screenshots: igdb_game.screenshots.map(|s| s.into_iter().filter_map(|sc| sc.url.map(|u| format!("https:{}", u.replace("t_thumb", "t_screenshot_big")))).collect()).unwrap_or_default(),
        artworks: igdb_game.artworks.map(|a| a.into_iter().filter_map(|ar| ar.url.map(|u| crate::domain::artwork::Artwork { url: format!("https:{}", u.replace("t_thumb", "t_1080p")) })).collect()).unwrap_or_default(),
    }
}

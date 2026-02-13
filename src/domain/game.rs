use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::domain::artwork::Artwork;

/// Representa un videojuego en el dominio de la aplicación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// El identificador único del juego.
    pub id: i64,

    /// El nombre del juego.
    pub name: String,

    /// El resumen detallado del juego.
    pub summary: Option<String>,

    /// Una breve descripción de la historia del juego.
    pub storyline: Option<String>,

    /// La fecha de lanzamiento.
    #[serde(rename = "release_date")]
    pub release_date: Option<NaiveDate>,

    /// Puntuación media de los usuarios (0-100).
    pub rating: Option<f64>,

    /// La URL de la imagen de portada.
    #[serde(rename = "cover_image_url")]
    pub cover_image_url: Option<String>,

    /// Lista de nombres de plataformas donde está disponible.
    pub platforms: Vec<String>,

    /// Una lista de géneros a los que pertenece el juego.
    pub genres: Vec<String>,

    /// Lista de URLs de videos relacionados (trailers, gameplays).
    pub videos: Vec<String>,

    /// Lista de URLs de capturas de pantalla.
    pub screenshots: Vec<String>,

    /// Una lista de artworks asociados al juego.
    pub artworks: Vec<Artwork>,
}

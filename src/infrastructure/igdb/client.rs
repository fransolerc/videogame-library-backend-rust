use reqwest::{Client, StatusCode};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, Duration};
use serde::de::DeserializeOwned;
use crate::infrastructure::igdb::dtos::TwitchTokenResponse;

#[derive(Clone)]
pub struct IgdbClient {
    client: Client,
    client_id: String,
    client_secret: String,
    base_url: String,
    auth_url: String,
    token: Arc<RwLock<Option<String>>>,
    token_expiry: Arc<RwLock<SystemTime>>,
}

impl IgdbClient {
    pub fn new(client_id: String, client_secret: String, base_url: String, auth_url: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            base_url,
            auth_url,
            token: Arc::new(RwLock::new(None)),
            token_expiry: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
        }
    }

    async fn get_token(&self) -> Result<String, String> {
        let now = SystemTime::now();
        let expiry = *self.token_expiry.read().await;

        if now < expiry {
            if let Some(token) = self.token.read().await.as_ref() {
                return Ok(token.clone());
            }
        }

        // Refresh token
        let params = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("grant_type", &"client_credentials".to_string()),
        ];

        let response = self.client.post(&self.auth_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!("Failed to get Twitch token: {}", response.status()));
        }

        let token_response: TwitchTokenResponse = response.json().await.map_err(|e| e.to_string())?;

        let mut token_guard = self.token.write().await;
        *token_guard = Some(token_response.access_token.clone());

        let mut expiry_guard = self.token_expiry.write().await;
        *expiry_guard = SystemTime::now() + Duration::from_secs(token_response.expires_in as u64 - 60); // Buffer of 60s

        Ok(token_response.access_token)
    }

    // Generic post that returns T (can be Vec<Something> or just Something)
    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, body: String) -> Result<T, String> {
        let token = self.get_token().await?;
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self.client.post(&url)
            .header("Client-ID", &self.client_id)
            .header("Authorization", format!("Bearer {}", token))
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status() == StatusCode::UNAUTHORIZED {
            let mut expiry_guard = self.token_expiry.write().await;
            *expiry_guard = SystemTime::UNIX_EPOCH;
            return Err("Unauthorized access to IGDB. Token might be expired.".to_string());
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("IGDB API Error: {} - {}", endpoint, error_text));
        }

        let response_text = response.text().await.map_err(|e| e.to_string())?;

        // Try to deserialize
        serde_json::from_str::<T>(&response_text)
            .map_err(|e| format!("JSON Deserialization Error: {} for response: {}", e, response_text))
    }
}

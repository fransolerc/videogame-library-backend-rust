use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Email
    pub user_id: String, // User ID
    pub exp: usize, // Expiration
}

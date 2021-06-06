use anyhow::{Context, Result};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpotifyConfig {
    #[serde(rename = "spotify_client_id")]
    pub client_id: String,

    #[serde(rename = "spotify_client_secret")]
    pub client_secret: String,

    #[serde(rename = "spotify_access_token")]
    pub access_token: String,

    #[serde(rename = "spotify_refresh_token")]
    pub refresh_token: String,
}

impl SpotifyConfig {
    pub fn from_env() -> Result<Self> {
        envy::from_env::<Self>()
            .with_context(|| "Failed to fetch configuration")
    }
}

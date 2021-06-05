use anyhow::{Context, Result};
use reqwest::Client;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyConfig {
    #[serde(rename = "spotify_client_id")]
    client_id: String,

    #[serde(rename = "spotify_client_secret")]
    client_secret: String,

    #[serde(rename = "spotify_access_token")]
    access_token: String,

    #[serde(rename = "spotify_refresh_token")]
    refresh_token: String,
}

impl SpotifyConfig {
    fn from_env() -> Result<Self> {
        envy::from_env::<Self>()
            .with_context(|| "Failed to fetch configuration")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = SpotifyConfig::from_env()?;

    let client = Client::new();
    let parameters = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &config.refresh_token),
    ];
    let response = client.post("https://accounts.spotify.com/api/token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .form(&parameters)
        .send()
        .await?;

    if response.status().is_success() {
        let token = response.json::<TokenResponse>().await?;
        println!("SPOTIFY_ACCESS_TOKEN='{}'", token.access_token);
        println!("SPOTIFY_REFRESH_TOKEN='{}'", token.refresh_token);
        Ok(())
    } else {
        let body = response.text().await?;
        eprintln!("{}", body);
        Ok(())
    }
}

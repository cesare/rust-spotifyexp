use anyhow::Result;
use reqwest::Client;
use serde_derive::Deserialize;

use spotifyexp::config::SpotifyConfig;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
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

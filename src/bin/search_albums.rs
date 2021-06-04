use anyhow::{Context, Result};
use reqwest::{Client};

fn spotify_access_token() -> Result<String> {
    std::env::var("SPOTIFY_ACCESS_TOKEN")
        .with_context(|| "envvar SPOTIFY_ACCESS_TOKEN missing")
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let parameters = [
        ("q", "pat metheny"),
        ("type", "album"),
        ("market", "from_token"),
        ("limit", "50"),

    ];
    let token = spotify_access_token()?;
    let response = client.get("https://api.spotify.com/v1/search")
        .header("Authorization", format!("Bearer {}", token))
        .query(&parameters)
        .send()
        .await?;
    println!("{:?}", response);
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

use anyhow::Result;
use reqwest::{Client};
use serde_derive::Deserialize;
use structopt::StructOpt;

use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "search_albums")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
}

#[derive(Debug, Deserialize)]
struct Artist {
    id: String,
    href: String,
    name: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct Album {
    id: String,
    href: String,
    artists: Vec<Artist>,
    name: String,
    release_date: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct Paging {
    href: String,
    items: Vec<Album>,
    limit: u32,
    offset: u32,
    total: u32,
    next: Option<String>,
    previous: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    albums: Paging,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: Error,
}

#[derive(Debug, Deserialize)]
struct Error {
    status: u32,
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = SpotifyConfig::from_env()?;

    let client = Client::new();
    let parameters = [
        ("q", arguments.query.as_str()),
        ("type", "album"),
        ("market", "from_token"),
        ("limit", "50"),
    ];
    let response = client.get("https://api.spotify.com/v1/search")
        .header("Authorization", format!("Bearer {}", config.access_token))
        .query(&parameters)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.json::<SearchResponse>().await?;
        println!("{:?}", body);
    } else {
        let error = response.json::<ErrorResponse>().await?;
        println!("{:?}", error);
    }
    Ok(())
}

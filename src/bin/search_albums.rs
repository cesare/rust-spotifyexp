use anyhow::Result;
use reqwest::{Client};
use structopt::StructOpt;

use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "search_albums")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
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

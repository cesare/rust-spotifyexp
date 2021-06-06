use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use structopt::StructOpt;

use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "search_albums")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
}

struct SearchAlbums {
    config: Rc<SpotifyConfig>,
    query: String,
}

impl SearchAlbums {
    fn new(config: &Rc<SpotifyConfig>, query: &str) -> Self {
        Self {
            config: config.clone(),
            query: query.to_owned(),
        }
    }

    async fn execute(&self) -> Result<SearchResponse> {
        let client = Client::new();
        let parameters = [
            ("q", self.query.as_str()),
            ("type", "album"),
            ("market", "from_token"),
            ("limit", "50"),
        ];
        let response = client.get("https://api.spotify.com/v1/search")
            .header("Authorization", format!("Bearer {}", self.config.access_token))
            .query(&parameters)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<SearchResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let search = SearchAlbums::new(&config, &arguments.query);
    let response = search.execute().await?;
    println!("{:?}", response);

    Ok(())
}

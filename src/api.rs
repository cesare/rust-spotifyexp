use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;

use crate::config::SpotifyConfig;
use crate::objects::*;

pub struct SearchAlbums {
    config: Rc<SpotifyConfig>,
    query: String,
}

impl SearchAlbums {
    pub fn new(config: &Rc<SpotifyConfig>, query: &str) -> Self {
        Self {
            config: config.clone(),
            query: query.to_owned(),
        }
    }

    pub async fn execute(&self) -> Result<SearchResponse> {
        let client = Client::new();
        let parameters = [
            ("q", self.query.as_str()),
            ("type", "album"),
            ("market", "from_token"),
            ("limit", "50"),
        ];
        let response = client.get("https://api.spotify.com/v1/search")
            .bearer_auth(&self.config.access_token)
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

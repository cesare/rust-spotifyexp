use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;

use crate::config::SpotifyConfig;
use crate::objects::{ErrorResponse, GetPlaylistsResponse};

struct GetPlaylists {
    config: Rc<SpotifyConfig>,
}

impl GetPlaylists {
    fn new(config: &Rc<SpotifyConfig>) -> Self {
        Self {
            config: config.clone(),
        }
    }

    async fn execute(&self) -> Result<GetPlaylistsResponse> {
        let client = Client::new();
        let response = client.get("https://api.spotify.com/v1/me/playlists")
            .bearer_auth(&self.config.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<GetPlaylistsResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

pub async fn get_playlists(config: &Rc<SpotifyConfig>) -> Result<GetPlaylistsResponse> {
    GetPlaylists::new(config).execute().await
}

use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;

use crate::config::SpotifyConfig;
use crate::objects::*;

mod player;
pub use self::player::{enqueue_tracks, is_playing, skip_to_next, start_playing};

mod search;
pub use self::search::{SearchAlbums, SearchArtists};

pub struct ListTracks {
    config: Rc<SpotifyConfig>,
    album_id: String,
}

impl ListTracks {
    pub fn new(config: &Rc<SpotifyConfig>, album_id: &str) -> Self {
        Self {
            config: config.clone(),
            album_id: album_id.to_owned(),
        }
    }

    pub async fn execute(&self) -> Result<ListTracksResponse> {
        let client = Client::new();
        let request_uri = format!("https://api.spotify.com/v1/albums/{}/tracks", self.album_id);
        let parameters = [
            ("market", "from_token"),
            ("limit", "50"),
        ];
        let response = client.get(request_uri)
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<ListTracksResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

pub struct ListDevices {
    config: Rc<SpotifyConfig>,
}

impl ListDevices {
    pub fn new(config: &Rc<SpotifyConfig>) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn execute(&self) -> Result<ListDevicesResponse> {
        let client = Client::new();
        let response = client.get("https://api.spotify.com/v1/me/player/devices")
            .bearer_auth(&self.config.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<ListDevicesResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

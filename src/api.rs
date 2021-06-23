use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;

use crate::config::SpotifyConfig;
use crate::objects::*;

mod player;
use self::player::*;

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

    pub async fn execute(&self) -> Result<SearchAlbumsResponse> {
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
            response.json::<SearchAlbumsResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

pub struct SearchArtists {
    config: Rc<SpotifyConfig>,
    query: String,
}

impl SearchArtists {
    pub fn new(config: &Rc<SpotifyConfig>, query: &str) -> Self {
        Self {
            config: config.clone(),
            query: query.to_owned(),
        }
    }

    pub async fn execute(&self) -> Result<SearchArtistsResponse> {
        let client = Client::new();
        let parameters = [
            ("q", self.query.as_str()),
            ("type", "artist"),
            ("market", "from_token"),
            ("limit", "50"),
        ];
        let response = client.get("https://api.spotify.com/v1/search")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<SearchArtistsResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            let e = response.json::<ErrorResponse>().await?;
            bail!("Request failed: {}", e.error.message)
        }
    }
}

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


pub async fn is_playing(config: &Rc<SpotifyConfig>) -> Result<bool> {
    let response = GetCurrentlyPlayingTrack::new(config).execute().await?;
    Ok(response.is_playing)
}

pub async fn enqueue_tracks(config: &Rc<SpotifyConfig>, device_id: &str, track_uris: Vec<String>) -> Result<()> {
    for uri in track_uris.iter() {
        EnqueueTrack::new(&config, device_id, &uri)
            .execute()
            .await?;
    }

    Ok(())
}

pub async fn skip_to_next(config: &Rc<SpotifyConfig>, device_id: &str) -> Result<()> {
    SkipToNextTrack::new(config, device_id)
        .execute()
        .await
}

pub async fn start_playing(config: &Rc<SpotifyConfig>, device_id: &str) -> Result<()> {
    StartPlaying::new(&config, &device_id)
        .execute()
        .await
}

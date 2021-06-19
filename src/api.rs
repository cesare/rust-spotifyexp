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

pub struct StartPlaying {
    config: Rc<SpotifyConfig>,
    device_id: String,
}

impl StartPlaying {
    pub fn new(config: &Rc<SpotifyConfig>, device_id: &str) -> Self {
        Self {
            config: config.clone(),
            device_id: device_id.to_owned(),
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let client = Client::new();
        let parameters = [
            ("device_id", &self.device_id),
        ];
        let response = client.put("https://api.spotify.com/v1/me/player/play")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            bail!("Request failed: {}", response.status())
        }
    }
}

pub struct EnqueueTrack {
    config: Rc<SpotifyConfig>,
    device_id: String,
    uri: String,
}

impl EnqueueTrack {
    pub fn new(config: &Rc<SpotifyConfig>, device_id: &str, uri: &str) -> Self {
        Self {
            config: config.clone(),
            device_id: device_id.to_owned(),
            uri: uri.to_owned(),
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let client = Client::new();
        let parameters = [
            ("device_id", &self.device_id),
            ("uri", &self.uri),
        ];
        let response = client.post("https://api.spotify.com/v1/me/player/queue")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            bail!("Request failed: {}", response.status())
        }
    }
}

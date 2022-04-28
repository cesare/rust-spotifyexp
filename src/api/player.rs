use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde_json::json;

use crate::config::SpotifyConfig;
use crate::objects::{CurrentlyPlayingTrackResponse, ErrorResponse, ListDevicesResponse};

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

pub struct GetCurrentlyPlayingTrack {
    config: Rc<SpotifyConfig>,
}

impl GetCurrentlyPlayingTrack {
    pub fn new(config: &Rc<SpotifyConfig>) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn execute(&self) -> Result<CurrentlyPlayingTrackResponse> {
        let client = Client::new();
        let parameters = [
            ("market", "from_token"),
        ];
        let response = client.get("https://api.spotify.com/v1/me/player/currently-playing")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .send()
            .await?;

        if response.status().is_success() {
            response.json::<CurrentlyPlayingTrackResponse>().await
                .with_context(|| "Failed to parse response")
        } else {
            bail!("Request failed: {}", response.status())
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

struct Playback {
    config: Rc<SpotifyConfig>,
    device_id: String,
    uri: String,
}

impl Playback {
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
        ];
        let body = json!({
            "context_uri": self.uri,
        });
        let response = client.put("https://api.spotify.com/v1/me/player/play")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
            .header("Content-Type", "application/json")
            .body(body.to_string())
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

pub struct SkipToNextTrack {
    config: Rc<SpotifyConfig>,
    device_id: String,
}

impl SkipToNextTrack {
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
        let response = client.post("https://api.spotify.com/v1/me/player/next")
            .bearer_auth(&self.config.access_token)
            .form(&parameters)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            bail!("Request failed: {}", response.status())
        }
    }
}

struct PausePlayback {
    config: Rc<SpotifyConfig>,
    device_id: String,
}

impl PausePlayback {
    fn new(config: &Rc<SpotifyConfig>, device_id: &str) -> Self {
        Self {
            config: config.clone(),
            device_id: device_id.to_owned(),
        }
    }

    async fn execute(&self) -> Result<()> {
        let client = Client::new();
        let parameters = [
            ("device_id", &self.device_id),
        ];
        let response = client.put("https://api.spotify.com/v1/me/player/pause")
            .bearer_auth(&self.config.access_token)
            .query(&parameters)
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

pub async fn get_currently_playing_track(config: &Rc<SpotifyConfig>) -> Result<CurrentlyPlayingTrackResponse> {
    GetCurrentlyPlayingTrack::new(config)
        .execute()
        .await
}

pub async fn is_playing(config: &Rc<SpotifyConfig>) -> Result<bool> {
    let response = GetCurrentlyPlayingTrack::new(config).execute().await?;
    Ok(response.is_playing)
}

pub async fn enqueue_tracks(config: &Rc<SpotifyConfig>, device_id: &str, track_uris: Vec<String>) -> Result<()> {
    for uri in track_uris.iter() {
        EnqueueTrack::new(config, device_id, uri)
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
    StartPlaying::new(config, device_id)
        .execute()
        .await
}

pub async fn playback(config: &Rc<SpotifyConfig>, device_id: &str, uri: &str) -> Result<()> {
    Playback::new(config, device_id, uri)
        .execute()
        .await
}

pub async fn pause(config: &Rc<SpotifyConfig>, device_id: &str) -> Result<()> {
    PausePlayback::new(config, device_id)
        .execute()
        .await
}

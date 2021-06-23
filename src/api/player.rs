use std::rc::Rc;

use anyhow::{Context, Result, bail};
use reqwest::Client;

use crate::config::SpotifyConfig;
use crate::objects::CurrentlyPlayingTrackResponse;

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

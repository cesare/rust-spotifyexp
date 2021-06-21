use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::{EnqueueTrack, GetCurrentlyPlayingTrack, SkipToNextTrack, StartPlaying};
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "play")]
struct Arguments {
    #[structopt(short, long)]
    device_id: String,

    #[structopt(short, long)]
    uri: Vec<String>,
}

async fn is_playing(config: &Rc<SpotifyConfig>) -> Result<bool> {
    let response = GetCurrentlyPlayingTrack::new(config).execute().await?;
    Ok(response.is_playing)
}

async fn skip_to_next(config: &Rc<SpotifyConfig>, device_id: &str) -> Result<()> {
    SkipToNextTrack::new(config, device_id)
        .execute()
        .await
}

async fn start_playing(config: &Rc<SpotifyConfig>, device_id: &str) -> Result<()> {
    StartPlaying::new(&config, &device_id)
        .execute()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    for uri in arguments.uri.iter() {
        EnqueueTrack::new(&config, &arguments.device_id, &uri)
            .execute()
            .await?;
    }

    let playing = is_playing(&config).await?;
    skip_to_next(&config, &arguments.device_id).await?;

    if !playing {
        start_playing(&config, &arguments.device_id).await?;
    }

    Ok(())
}

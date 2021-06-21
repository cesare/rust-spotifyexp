use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::{EnqueueTrack, GetCurrentlyPlayingTrack, StartPlaying};
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

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let playing = is_playing(&config).await?;

    for uri in arguments.uri.iter() {
        EnqueueTrack::new(&config, &arguments.device_id, &uri)
            .execute()
            .await?;
    }

    StartPlaying::new(&config, &arguments.device_id)
        .execute()
        .await?;

    Ok(())
}

use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::{enqueue_tracks, is_playing, skip_to_next, start_playing};
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "play")]
struct Arguments {
    #[structopt(short, long)]
    device_id: String,

    #[structopt(short, long)]
    uri: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    enqueue_tracks(&config, &arguments.device_id, arguments.uri).await?;

    let playing = is_playing(&config).await?;
    skip_to_next(&config, &arguments.device_id).await?;

    if !playing {
        start_playing(&config, &arguments.device_id).await?;
    }

    Ok(())
}

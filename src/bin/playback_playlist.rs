use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::playback_playlist;
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "playback_playlist")]
struct Arguments {
    #[structopt(short, long)]
    device_id: String,

    #[structopt(short, long)]
    uri: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    playback_playlist(&config, &arguments.device_id, &arguments.uri).await
}

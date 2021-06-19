use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::{EnqueueTrack, StartPlaying};
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

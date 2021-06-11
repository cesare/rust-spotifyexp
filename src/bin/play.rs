use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::StartPlaying;
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "album_tracks")]
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

    let response = StartPlaying::new(&config, &arguments.device_id, &arguments.uri).execute().await?;
    println!("{:?}", response);
    Ok(())
}

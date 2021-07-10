use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::pause;
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "pause")]
struct Arguments {
    #[structopt(short, long)]
    device_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    pause(&config, &arguments.device_id).await
}

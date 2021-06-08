use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::ListTracks;
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "album_tracks")]
struct Arguments {
    #[structopt(short, long)]
    album_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let response = ListTracks::new(&config, &arguments.album_id)
        .execute()
        .await?;
    println!("{:?}", response);

    Ok(())
}

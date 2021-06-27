use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::list_tracks;
use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::Track;

#[derive(StructOpt, Debug)]
#[structopt(name = "album_tracks")]
struct Arguments {
    #[structopt(short, long)]
    album_id: String,
}

fn show_track(track: &Track) {
    println!("{} {}", track.uri, track.name);
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let response = list_tracks(&config, &arguments.album_id).await?;
    for track in response.items.iter() {
        show_track(&track);
    }

    Ok(())
}

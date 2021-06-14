use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::SearchArtists;
use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::{SearchArtistsResponse, Artist};

#[derive(StructOpt, Debug)]
#[structopt(name = "search_artists")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
}

fn show_artist(artist: &Artist) {
    println!("{} {}", artist.uri, artist.name);
}

fn show_results(response: &SearchArtistsResponse) {
    for artist in response.artists.items.iter() {
        show_artist(&artist);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let search = SearchArtists::new(&config, &arguments.query);
    let response = search.execute().await?;
    show_results(&response);

    Ok(())
}

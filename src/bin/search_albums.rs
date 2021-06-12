use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::SearchAlbums;
use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::{SearchResponse, Album};

#[derive(StructOpt, Debug)]
#[structopt(name = "search_albums")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
}

fn show_album(album: &Album) {
    println!("{} {}", album.uri, album.name);
}

fn show_results(response: &SearchResponse) {
    for album in response.albums.items.iter() {
        show_album(&album);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let search = SearchAlbums::new(&config, &arguments.query);
    let response = search.execute().await?;
    show_results(&response);

    Ok(())
}

use std::rc::Rc;

use anyhow::Result;

use spotifyexp::api::get_currently_playing_track;
use spotifyexp::config::SpotifyConfig;
use spotifyexp::objects::CurrentlyPlayingItem;

fn show_track(item: &CurrentlyPlayingItem) {
    let artists = item.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", ");
    println!("{} - {}", item.name, artists);
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Rc::new(SpotifyConfig::from_env()?);

    let response = get_currently_playing_track(&config).await?;
    show_track(&response.item);

    Ok(())
}

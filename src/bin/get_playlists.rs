use std::rc::Rc;

use anyhow::Result;

use spotifyexp::api::get_playlists;
use spotifyexp::config::SpotifyConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Rc::new(SpotifyConfig::from_env()?);
    let response = get_playlists(&config).await?;
    println!("{:?}", response.items);
    Ok(())
}

use std::rc::Rc;

use anyhow::Result;

use spotifyexp::api::get_currently_playing_track;
use spotifyexp::config::SpotifyConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Rc::new(SpotifyConfig::from_env()?);

    let response = get_currently_playing_track(&config).await?;
    println!("{:?}", response);

    Ok(())
}

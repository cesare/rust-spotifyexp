use std::rc::Rc;

use anyhow::Result;

use spotifyexp::api::ListDevices;
use spotifyexp::config::SpotifyConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Rc::new(SpotifyConfig::from_env()?);
    let response = ListDevices::new(&config).execute().await?;
    println!("{:?}", response);
    Ok(())
}

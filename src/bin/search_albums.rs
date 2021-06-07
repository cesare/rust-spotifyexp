use std::rc::Rc;

use anyhow::Result;
use structopt::StructOpt;

use spotifyexp::api::SearchAlbums;
use spotifyexp::config::SpotifyConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "search_albums")]
struct Arguments {
    #[structopt(short, long)]
    query: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::from_args();
    let config = Rc::new(SpotifyConfig::from_env()?);

    let search = SearchAlbums::new(&config, &arguments.query);
    let response = search.execute().await?;
    println!("{:?}", response);

    Ok(())
}

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
    id: String,
    href: String,
    name: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    id: String,
    href: String,
    artists: Vec<Artist>,
    name: String,
    release_date: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    id: String,
    href: String,
    artists: Vec<Artist>,
    name: String,
    disc_number: u32,
    track_number: u32,
    duration_ms: u32,
    uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Paging<T> {
    href: String,
    items: Vec<T>,
    limit: u32,
    offset: u32,
    total: u32,
    next: Option<String>,
    previous: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    albums: Paging<Album>,
}

pub type ListTracksResponse = Paging<Track>;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub status: u32,
    pub message: String,
}

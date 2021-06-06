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
pub struct Paging {
    href: String,
    items: Vec<Album>,
    limit: u32,
    offset: u32,
    total: u32,
    next: Option<String>,
    previous: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    albums: Paging,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    error: Error,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    status: u32,
    message: String,
}

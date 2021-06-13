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
    pub id: String,
    pub href: String,
    pub artists: Vec<Artist>,
    pub name: String,
    pub release_date: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: String,
    pub href: String,
    pub artists: Vec<Artist>,
    pub name: String,
    pub disc_number: u32,
    pub track_number: u32,
    pub duration_ms: u32,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Paging<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub albums: Paging<Album>,
}

pub type ListTracksResponse = Paging<Track>;

#[derive(Debug, Deserialize)]
pub struct ListDevicesResponse {
    devices: Vec<Device>,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    id: String,
    is_active: bool,
    is_private_session: bool,
    is_restricted: bool,
    name: String,
    #[serde(rename = "type")]
    device_type: String,
    volume_percent: u32,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub status: u32,
    pub message: String,
}

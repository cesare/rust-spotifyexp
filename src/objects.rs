use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: String,
    pub href: String,
    pub name: String,
    pub uri: String,
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
pub struct SearchAlbumsResponse {
    pub albums: Paging<Album>,
}

#[derive(Debug, Deserialize)]
pub struct SearchArtistsResponse {
    pub artists: Paging<Artist>,
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
pub struct CurrentlyPlayingTrackResponse {
    pub timestamp: u64,
    pub is_playing: bool,
    pub currently_playing_type: String,
    pub item: CurrentlyPlayingItem,
}

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingItem {
    pub album: Option<Album>,
    pub artists: Vec<Artist>,
    pub name: String,
}

pub type GetPlaylistsResponse = Paging<Playlist>;

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub description: String,
    pub href: String,
    pub name: String,
    pub tracks: PlaylistTracksRef,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTracksRef {
    pub href: String,
    pub total: u32,
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

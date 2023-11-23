use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SubsonicSearch3Response {
    pub subsonic_response: SubsonicResponse,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResponse {
    pub status: String,
    pub version: String,
    #[serde(rename = "type")]
    pub subsonic_response_type: String,
    pub server_version: String,
    pub search_result3: Option<SearchResult3>,
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct  SearchResult3 {
    artist: Vec<Artist>,
    album: Vec<Album>,
    song: Vec<Song>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    id: String,
    parent: String,
    is_dir: bool,
    title: String,
    name: String,
    album: String,
    artist: String,
    year: i64,
    cover_art: String,
    duration: i64,
    play_count: i64,
    played: String,
    created: String,
    artist_id: String,
    song_count: i64,
    is_video: bool,
    genre: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    id: String,
    name: String,
    cover_art: String,
    album_count: i64,
    artist_image_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    id: String,
    parent: String,
    is_dir: bool,
    title: String,
    album: String,
    artist: String,
    track: i64,
    year: i64,
    cover_art: String,
    size: i64,
    content_type: String,
    suffix: String,
    duration: i64,
    bit_rate: i64,
    path: String,
    disc_number: Option<i64>,
    created: String,
    album_id: String,
    artist_id: Option<String>,
    #[serde(rename = "type")]
    song_type: String,
    is_video: bool,
    genre: Option<String>,
    play_count: Option<i64>,
    played: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

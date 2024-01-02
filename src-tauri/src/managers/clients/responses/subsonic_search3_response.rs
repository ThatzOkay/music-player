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
    pub artist: Option<Vec<Artist>>,
    pub album: Option<Vec<Album>>,
    pub song: Option<Vec<Song>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: String,
    pub parent: String,
    pub is_dir: bool,
    pub title: String,
    pub name: String,
    pub album: String,
    pub artist: String,
    pub year: i64,
    pub cover_art: String,
    pub duration: i64,
    pub play_count: i64,
    pub played: String,
    pub created: String,
    pub artist_id: String,
    pub song_count: i64,
    pub is_video: bool,
    pub genre: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub cover_art: String,
    pub album_count: i64,
    pub artist_image_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub parent: String,
    pub is_dir: bool,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub track: i64,
    pub year: i64,
    pub cover_art: String,
    pub size: i64,
    pub content_type: String,
    pub suffix: String,
    pub duration: i64,
    pub bit_rate: i64,
    pub path: String,
    pub disc_number: Option<i64>,
    pub created: String,
    pub album_id: String,
    pub artist_id: Option<String>,
    #[serde(rename = "type")]
    pub song_type: String,
    pub is_video: bool,
    pub genre: Option<String>,
    pub play_count: Option<i64>,
    pub played: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

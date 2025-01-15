
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SubsonicAlbumResponse {
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
    pub open_subsonic: bool,
    pub album_list2: AlbumList2,
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumList2 {
    pub album: Vec<Album>,
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
    pub genre: Option<String>,
    pub cover_art: String,
    pub duration: i64,
    pub play_count: i64,
    pub created: String,
    pub artist_id: String,
    pub song_count: i64,
    pub is_video: bool,
    pub played: String,
    pub bpm: i64,
    pub comment: String,
    pub sort_name: String,
    pub media_type: MediaType,
    pub music_brainz_id: String,
    pub genres: Vec<Option<serde_json::Value>>,
    pub replay_gain: ReplayGain,
    pub channel_count: i64,
    pub sampling_rate: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Album,
}

#[derive(Serialize, Deserialize)]
pub struct ReplayGain {
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}
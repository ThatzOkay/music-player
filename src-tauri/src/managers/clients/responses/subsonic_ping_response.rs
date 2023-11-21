use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SubsonicPingResponse {
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
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

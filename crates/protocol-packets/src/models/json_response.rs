use protocol_core::data::Json;
use serde_json::Value;

pub type JsonResponse = Json<JsonResponseContent>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JsonResponseContent {
    pub version: Version,
    pub players: Players,
    pub description: Value,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
    #[serde(rename = "previewsChat")]
    pub previews_chat: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Option<Vec<Player>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}

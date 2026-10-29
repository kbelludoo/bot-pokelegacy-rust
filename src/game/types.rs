use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub token: String,
    #[serde(rename = "deviceUuid", default)]
    pub device_uuid: String,
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    #[serde(rename = "huntTarget", default)]
    pub hunt_target: String,
    #[serde(rename = "autoHunt", default = "default_true")]
    pub auto_hunt: bool,
    #[serde(rename = "autoCatch", default = "default_true")]
    pub auto_catch: bool,
    #[serde(rename = "autoSell", default = "default_true")]
    pub auto_sell: bool,
    #[serde(rename = "autoDaily", default = "default_true")]
    pub auto_daily: bool,
    #[serde(rename = "switchWeakness", default = "default_true")]
    pub switch_weakness: bool,
    #[serde(rename = "ballId", default = "default_ball")]
    pub ball_id: u32,
    #[serde(rename = "shinyBallId", default = "default_shiny_ball")]
    pub shiny_ball_id: u32,
}

fn default_true() -> bool {
    true
}

fn default_ball() -> u32 {
    1
}

fn default_shiny_ball() -> u32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pokemon {
    pub id: String,
    #[serde(rename = "speciesId", default)]
    pub species_id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub hp: u32,
    #[serde(rename = "maxHp", default)]
    pub max_hp: u32,
    #[serde(default)]
    pub shiny: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerState {
    pub username: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub xp: u64,
    #[serde(default)]
    pub gold: i64,
    #[serde(default)]
    pub team: Vec<Pokemon>,
    #[serde(rename = "activeHunt", default)]
    pub active_hunt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginResponse {
    pub token: String,
    #[serde(default)]
    pub user: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeResponse {
    #[serde(default)]
    pub token: String,
    pub state: Option<PlayerState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveEnvelope {
    pub state: PlayerState,
    #[serde(rename = "clientTimestamp")]
    pub client_timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub version: u64,
    #[serde(default)]
    pub code: Option<String>,
}

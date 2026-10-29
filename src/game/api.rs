use crate::game::types::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use std::time::Duration;
use tracing::{debug, error, info};

pub const DEFAULT_CLIENT_RELEASE: &str = "64b3cbb920d113d0df6e8532699493e762b1332c";
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub struct LegacyClient {
    client: reqwest::Client,
    base_url: String,
    device_id: String,
    token: String,
}

impl LegacyClient {
    pub fn new(config: &AccountConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Client-Release", HeaderValue::from_static(DEFAULT_CLIENT_RELEASE));

        let device_id = if config.credentials.device_uuid.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            config.credentials.device_uuid.clone()
        };

        if let Ok(v) = HeaderValue::from_str(&device_id) {
            headers.insert("X-Poke-Aparelho", v);
        }

        let base_url = if config.credentials.origin.is_empty() {
            "https://pokelegacyonline.com.br".to_string()
        } else {
            config.credentials.origin.trim_end_matches('/').to_string()
        };

        if let Ok(v) = HeaderValue::from_str(&base_url) {
            headers.insert(ORIGIN, v);
        }
        let referer_str = format!("{}/jogar", base_url);
        if let Ok(v) = HeaderValue::from_str(&referer_str) {
            headers.insert(REFERER, v);
        }

        if !config.credentials.token.is_empty() {
            let auth = format!("Bearer {}", config.credentials.token);
            if let Ok(v) = HeaderValue::from_str(&auth) {
                headers.insert(AUTHORIZATION, v);
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(20))
            .build()
            .expect("LegacyClient");

        Self {
            client,
            base_url,
            device_id,
            token: config.credentials.token.clone(),
        }
    }

    pub async fn login(&self, email: &str, pass: &str) -> Result<LoginResponse, String> {
        let url = format!("{}/api/login", self.base_url);
        let req = LoginRequest {
            login: email.to_string(),
            password: pass.to_string(),
        };
        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Login error: {e}"))?;

        resp.json::<LoginResponse>()
            .await
            .map_err(|e| format!("Login decode error: {e}"))
    }

    pub async fn fetch_me(&self) -> Result<MeResponse, String> {
        let url = format!("{}/api/me", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Fetch me error: {e}"))?;

        resp.json::<MeResponse>()
            .await
            .map_err(|e| format!("Fetch me decode error: {e}"))
    }

    pub async fn push_save(&self, state: &PlayerState) -> Result<SaveResponse, String> {
        let url = format!("{}/api/save", self.base_url);
        let env = SaveEnvelope {
            state: state.clone(),
            client_timestamp: chrono::Utc::now().timestamp_millis(),
        };
        let resp = self
            .client
            .post(&url)
            .json(&env)
            .send()
            .await
            .map_err(|e| format!("Save error: {e}"))?;

        resp.json::<SaveResponse>()
            .await
            .map_err(|e| format!("Save decode error: {e}"))
    }

    pub async fn claim_daily_reward(&self) -> Result<bool, String> {
        let url = format!("{}/api/daily-reward/claim", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| format!("Claim error: {e}"))?;

        Ok(resp.status().is_success())
    }

    pub async fn sell_trash(&self) -> Result<bool, String> {
        let url = format!("{}/api/mall/sell", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "auto": true }))
            .send()
            .await
            .map_err(|e| format!("Sell error: {e}"))?;

        Ok(resp.status().is_success())
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct SrsCallback {
    pub action: String,
    pub client_id: String,
    pub ip: String,
    pub vhost: String,
    pub app: String,
    pub stream: String,
    pub param: String,
    pub stream_url: String,
    pub tc_url: String,
    pub page_url: Option<String>,
    pub stream_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SrsResponse {
    pub code: i32,
    pub data: Option<HashMap<String, String>>,
}

impl Default for SrsResponse {
    fn default() -> Self {
        Self {
            code: 0,
            data: None,
        }
    }
}

impl SrsResponse {
    pub fn success() -> Self {
        Self::default()
    }
    
    pub fn error(code: i32) -> Self {
        Self {
            code,
            data: None,
        }
    }
    
    pub fn with_data(mut self, data: HashMap<String, String>) -> Self {
        self.data = Some(data);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct SrsAuthRequest {
    pub action: String,
    pub client_id: String,
    pub ip: String,
    pub vhost: String,
    pub app: String,
    pub stream: String,
    pub param: String,
    pub stream_url: String,
    pub tc_url: String,
    pub page_url: Option<String>,
}

impl SrsAuthRequest {
    pub fn extract_stream_key(&self) -> Option<String> {
        // 从stream URL中提取stream key
        // 格式通常是: rtmp://server/app/stream_key
        let parts: Vec<&str> = self.stream_url.split('/').collect();
        if parts.len() >= 4 {
            Some(parts[3].to_string())
        } else {
            None
        }
    }
}

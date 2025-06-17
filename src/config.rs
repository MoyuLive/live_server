use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,

    pub database_url: String,

    #[serde(alias = "srs")]
    pub srs_servers: Vec<SrsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: Option<String>,
    pub format: Option<String>,
    pub path: Option<String>,

    pub console: Option<bool>,
    #[serde(alias = "consolelevel")]
    pub console_level: Option<String>,
}

impl LogConfig {
    pub fn str_to_level<T: ToString>(l: T) -> tracing::Level {
        match l.to_string().to_lowercase().as_str() {
            "trace" | "tracing" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" | "warning" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        }
    }

    pub fn to_level<T: ToString>(l: Option<T>) -> tracing::Level {
        match l {
            Some(l) => Self::str_to_level(l),
            None => tracing::Level::INFO,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrsConfig {
    #[serde(alias = "api")]
    pub api_url: String,
    // TODO
}

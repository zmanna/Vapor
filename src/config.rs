use std::env;
use std::path::PathBuf;

pub const DEFAULT_API_BASE_URL: &str =
    "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api";
pub const DEFAULT_CHAT_SERVER_ADDR: &str = "127.0.0.1:8080";

/// Runtime configuration loaded from environment variables with safe defaults.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeConfig {
    pub api_base_url: String,
    pub chat_server_addr: String,
    pub library_path: Option<PathBuf>,
}

impl RuntimeConfig {
    /// Loads runtime configuration from the process environment.
    ///
    /// Supported variables:
    /// - `VAPOR_API_BASE_URL`
    /// - `VAPOR_CHAT_SERVER_ADDR`
    /// - `VAPOR_LIBRARY_PATH`
    pub fn from_env() -> Self {
        Self {
            api_base_url: env::var("VAPOR_API_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string()),
            chat_server_addr: env::var("VAPOR_CHAT_SERVER_ADDR")
                .unwrap_or_else(|_| DEFAULT_CHAT_SERVER_ADDR.to_string()),
            library_path: env::var_os("VAPOR_LIBRARY_PATH").map(PathBuf::from),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            api_base_url: DEFAULT_API_BASE_URL.to_string(),
            chat_server_addr: DEFAULT_CHAT_SERVER_ADDR.to_string(),
            library_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_uses_documented_service_locations() {
        let config = RuntimeConfig::default();

        assert_eq!(config.api_base_url, DEFAULT_API_BASE_URL);
        assert_eq!(config.chat_server_addr, DEFAULT_CHAT_SERVER_ADDR);
        assert_eq!(config.library_path, None);
    }
}

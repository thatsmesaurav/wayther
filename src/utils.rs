use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_api_key() -> Option<String> {
    if let Ok(key) = env::var("OPENWEATHERMAP_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }

    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("wayther").join("config.toml");
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = content.parse::<toml::Table>() {
                if let Some(key) = config.get("api_key").and_then(|v| v.as_str()) {
                    return Some(key.to_string());
                }
            }
        }
    }

    None
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wayther")
        .join("config.toml")
}

pub fn get_location() -> Option<String> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("wayther").join("config.toml");
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = content.parse::<toml::Table>() {
                if let Some(location) = config.get("location").and_then(|v| v.as_str()) {
                    return Some(location.to_string());
                }
            }
        }
    }
    None
}

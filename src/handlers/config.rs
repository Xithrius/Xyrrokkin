use anyhow::{bail, Error, Result};
use std::panic::panic_any;

const BINARY_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CompleteConfig {
    pub private_key_path: String,

    pub application_id: u64,

    pub application_token: String,

    #[serde(default = "default_server_uri")]
    pub server_uri: String,
}

fn default_server_uri() -> String {
    "127.0.0.1:3000".to_string()
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        if let Ok(config_contents) = std::fs::read_to_string(config_path()) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
                config_path(),
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}

pub fn config_path() -> String {
    match std::env::consts::OS {
        "linux" | "macos" => match std::env::var("HOME") {
            Ok(env_home_path) => format!("{}/.config/{}/config.toml", env_home_path, BINARY_NAME),
            Err(err) => panic_any(err),
        },
        "windows" => match std::env::var("APPDATA") {
            Ok(appdata_path) => format!("{}\\{}\\config.toml", appdata_path, BINARY_NAME),
            Err(err) => std::panic::panic_any(err),
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_config_path() {
        match std::env::var("APPDATA") {
            Ok(appdata_path) => assert_eq!(
                config_path(),
                format!("{}\\{}\\config.toml", appdata_path, BINARY_NAME)
            ),
            Err(err) => std::panic::panic_any(err),
        }
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn test_unix_config_path() {
        match std::env::var("HOME") {
            Ok(env_home_path) => assert_eq!(
                config_path(),
                format!("{}/.config/{}/config.toml", env_home_path, BINARY_NAME)
            ),
            Err(err) => std::panic::panic_any(err),
        }
    }
}

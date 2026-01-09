use anyhow::{Context, Result as AnyResult};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub css_file_path: Option<PathBuf>,
}

impl Config {
    pub fn load_from_file() -> AnyResult<Config> {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|_| {
                env::var("HOME")
                    .map(|home| PathBuf::from(home).join(".config"))
            })
            .context("Could not determine config directory")?;

        let config_folder_path = config_dir.join("llame");
        let config_file_path = config_folder_path.join("config.toml");

        let data_str = std::fs::read_to_string(config_file_path)?;
        let mut config: Config = toml::from_str(&data_str)?;

        if let Some(ref path) = config.css_file_path {
            if path.is_relative() {
                config.css_file_path = Some(config_folder_path.join(path));
            }
        }

        Ok(config)
    }
}

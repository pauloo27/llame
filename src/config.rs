use anyhow::{Context, Result as AnyResult};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub css_file_path: Option<PathBuf>,
}

const APP_CONFIG_FOLDER_PATH: &str = ".config/llame/";

impl Config {
    pub fn load_from_file() -> AnyResult<Config> {
        // FIXME: use a crate instead of std::home_dir since it's deprecated
        let user_home = env::home_dir().context("User directory not found")?;
        let config_folder_path = user_home.join(APP_CONFIG_FOLDER_PATH);
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

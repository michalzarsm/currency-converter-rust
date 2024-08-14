use std::env;
use directories::ProjectDirs;
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

#[derive(Error, Debug)]
enum ConfigError {
    #[error("No API key set.")]
    NotFound,
    #[error("Error reading config.")]
    ReadError,
    #[error("Error writing config.")]
    WriteError,
    #[error("Error removing config.")]
    RemoveError,
    #[error("JSON Parse Error.")]
    JsonParseError,
    #[error("Directory Error.")]
    DirectoryError,
}

pub fn get_config_dir() -> Result<String> {
    let project_dirs = match ProjectDirs::from("", "", "CurrencyConverter") {
        Some(dirs) => dirs,
        None => return Err(ConfigError::DirectoryError.into()),
    };
    let config_dir = match project_dirs.config_dir().to_str() {
        Some(dir) => dir.to_string(),
        None => return Err(ConfigError::DirectoryError.into()),
    };
    Ok(config_dir)
}

pub fn write_config(api_key: String) -> Result<()> {
    let config = Config { api_key };
    let config_dir = get_config_dir()?;
    let config_file = format!("{}/config.json", config_dir);
    let config_json = match serde_json::to_string(&config) {
        Ok(json) => json,
        Err(_) => return Err(ConfigError::JsonParseError.into()),
    };
    
    match fs::create_dir_all(config_dir) {
        Ok(_) => (),
        Err(_) => return Err(ConfigError::DirectoryError.into()),
    }

    match fs::write(config_file, config_json) {
        Ok(_) => Ok(()),
        Err(_) => return Err(ConfigError::WriteError.into()),
    }
}

pub fn remove_config() -> Result<()> {
    let config_dir = get_config_dir()?;
    let config_file = format!("{}/config.json", config_dir);
    if fs::metadata(&config_file).is_ok() {
        match fs::remove_file(config_file) {
            Ok(_) => Ok(()),
            Err(_) => return Err(ConfigError::RemoveError.into()),
        }
    } else {
        Err(ConfigError::NotFound.into())
    }
}

pub fn read_config() -> Result<Config> {
    let config_dir = get_config_dir()?;
    let config_file = format!("{}/config.json", config_dir);
    if fs::metadata(&config_file).is_ok() {
        let config_json = match fs::read_to_string(config_file) {
            Ok(json) => json,
            Err(_) => return Err(ConfigError::ReadError.into()),
        };
        let config: Config = match serde_json::from_str(&config_json) {
            Ok(config) => config,
            Err(_) => return Err(ConfigError::JsonParseError.into()),
        };
        Ok(config)
    } else {
        match env::var("API_KEY") {
            Ok(api_key) => return Ok(Config { api_key }),
            Err(_) => return Err(ConfigError::NotFound.into()),
        }
    }
}
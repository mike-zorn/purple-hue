use config::{Config, ConfigError, File};
use dirs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub light_id: usize,
    pub sensor_id: Option<u32>,
    pub user_id: String,
    pub sensor_ip: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("/etc/purple-hue").required(false))?;
        if let Some(config_dir) = dirs::config_dir() {
            if let Some(config_dir_str) = config_dir.join(PathBuf::from("purple-hue")).to_str() {
                s.merge(File::with_name(config_dir_str).required(false))?;
            }
        }
        s.merge(File::with_name("purple-hue").required(false))?;
        s.try_into()
    }
}

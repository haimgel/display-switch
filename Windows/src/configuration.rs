//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::{anyhow, Result};
use config;
use dirs;
use serde::Deserialize;
use std::fs;

use crate::display_control;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub usb_device: String,
    pub monitor_input: display_control::InputSource,
    pub on_connect: Option<HashMap<String, display_control::InputSource>>,
    pub on_disconnect: Option<HashMap<String, display_control::InputSource>>,
}

impl Configuration {
    pub fn load() -> Result<Self> {
        if let Ok(config) = Self::try_load_toml() {
            return Ok(config);
        }
        if let Ok(config) = Self::try_load_json() {
            return Ok(config);
        }

        Self::try_load_ini()
    }

    pub fn try_load_ini() -> Result<Self> {
        let config_file_name = Self::config_file_name("ini")?;
        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(config_file_name.to_path_buf()))?
            .merge(config::Environment::with_prefix("DISPLAY_SWITCH"))?;
        let config = settings.try_into::<Self>()?;
        info!(
            "Configuration loaded ({:?}): {:?}",
            config_file_name, config
        );
        Ok(config)
    }

    pub fn try_load_toml() -> Result<Self> {
        let config_data = Self::config_file_str("toml")?;
        let config = toml::from_str::<Self>(&config_data)?;

        info!("Configuration loaded from toml: {:?}", config);
        Ok(config)
    }

    pub fn try_load_json() -> Result<Self> {
        let config_data = Self::config_file_str("json")?;
        let config = serde_json::from_str::<Self>(&config_data)?;

        info!("Configuration loaded from toml: {:?}", config);
        Ok(config)
    }

    pub fn config_file_name(extension: &str) -> Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or(anyhow!("Config directory not found"))?
            .join("display-switch");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join(format!("display-switch.{}", extension)))
    }

    pub fn config_file_str(extension: &str) -> Result<String> {
        let config_file_name = Self::config_file_name(extension)?;
        let config_data = fs::read_to_string(&config_file_name)?;

        info!("Configuration file found {:?}", config_file_name);
        Ok(config_data)
    }

    pub fn log_file_name() -> Result<std::path::PathBuf> {
        let log_dir = dirs::data_local_dir()
            .ok_or(anyhow!("Data-local directory not found"))?
            .join("display-switch");
        std::fs::create_dir_all(&log_dir)?;
        Ok(log_dir.join("display-switch.log"))
    }
}

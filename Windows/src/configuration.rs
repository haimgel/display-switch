use config;
use dirs;
use serde::Deserialize;
use anyhow::{anyhow, Result};

use crate::display_control;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub usb_device: String,
    pub monitor_input: display_control::InputSource
}

impl Configuration {
    pub fn load() -> Result<Self> {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(Self::config_file_name()?))?
            .merge(config::Environment::with_prefix("DISPLAY_SWITCH"))?;
        Ok(settings.try_into::<Self>()?)
    }

    fn config_file_name() -> Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or(anyhow!("Config directory not found"))?
            .join("display-switch");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("display-switch.ini"))
    }
}

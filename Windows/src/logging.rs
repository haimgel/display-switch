use std::fs::File;
use simplelog::*;
use anyhow::Result;

use crate::configuration::Configuration;

pub fn init_logging() -> Result<()> {
    Ok(CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Debug, Config::default(),
                File::create(Configuration::log_file_name()?)?
            )
        ]
    )?)
}

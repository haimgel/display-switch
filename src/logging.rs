//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use std::fs::File;

use anyhow::Result;
use simplelog::*;

use crate::configuration::Configuration;

pub fn init_logging(log_debug: bool) -> Result<()> {
    let log_level = if log_debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    Ok(CombinedLogger::init(vec![
        TermLogger::new(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(
            log_level,
            Config::default(),
            File::create(Configuration::log_file_name()?)?,
        ),
    ])?)
}

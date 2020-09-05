//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use std::fs::File;

use anyhow::Result;
use simplelog::*;

use crate::configuration::Configuration;

pub fn init_logging() -> Result<()> {
    Ok(CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(Configuration::log_file_name()?)?,
        ),
    ])?)
}

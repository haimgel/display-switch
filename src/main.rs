//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Parser;

#[cfg(target_os = "windows")]
use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};

mod app;
mod configuration;
mod display_control;
mod input_source;
mod logging;
mod platform;
mod usb;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Print debug information
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Path to an alternative configuration file
    #[arg(short = 'c', long = "config")]
    config_file_path: Option<std::path::PathBuf>,
}

/// On Windows, re-attach the console, if parent process has the console. This allows
/// to see the log output when run from the command line.
fn attach_console() {
    #[cfg(target_os = "windows")]
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

fn main() -> Result<()> {
    attach_console();
    let args = Args::parse();

    let app = app::App::new(args)?;
    app.run()?;
    Ok(())
}

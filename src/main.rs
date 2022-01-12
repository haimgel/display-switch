//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

use anyhow::Result;
use std::env;

#[cfg(target_os = "windows")]
use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};

mod app;
mod configuration;
mod display_control;
mod input_source;
mod logging;
mod platform;
mod usb;

/// On Windows, re-attach the console, if parent process has the console. This allows
/// to see the log output when run from the command line.
fn attach_console() {
    #[cfg(target_os = "windows")]
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "--version" {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    attach_console();
    let app = app::App::new()?;
    app.run()?;
    Ok(())
}

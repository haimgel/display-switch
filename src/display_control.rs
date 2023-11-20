//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use crate::configuration::{Configuration, SwitchDirection};
use crate::input_source::InputSource;

use anyhow::{Error, Result};
use ddc_hi::{Ddc, Display, Handle};

use std::collections::HashSet;
use std::process::{Command, Stdio};
use std::{thread, time};

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;
const RETRY_DELAY_MS: u64 = 3000;

fn display_name(display: &Display, index: Option<usize>) -> String {
    // Different OSes populate different fields of ddc-hi-rs info structure differently. Create
    // a synthetic "display_name" that makes sense on each OS
    #[cfg(target_os = "linux")]
    let display_id = vec![
        &display.info.manufacturer_id,
        &display.info.model_name,
        &display.info.serial_number,
    ]
    .into_iter()
    .flatten()
    .map(|s| s.as_str())
    .collect::<Vec<&str>>()
    .join(" ");
    #[cfg(target_os = "macos")]
    let display_id = &display.info.id;
    #[cfg(target_os = "windows")]
    let display_id = &display.info.id;

    if let Some(index) = index {
        format!("'{} #{}'", display_id, index)
    } else {
        format!("'{}'", display_id)
    }
}

fn are_display_names_unique(displays: &[Display]) -> bool {
    let mut hash = HashSet::new();
    displays.iter().all(|display| hash.insert(display_name(display, None)))
}

fn try_switch_display(handle: &mut Handle, display_name: &str, input: InputSource) {
    match handle.get_vcp_feature(INPUT_SELECT) {
        Ok(raw_source) => {
            if raw_source.value() & 0xff == input.value() {
                info!("Display {} is already set to {}", display_name, input);
                return;
            }
        }
        Err(err) => {
            warn!("Failed to get current input for display {}: {:?}", display_name, err);
        }
    }
    debug!("Setting display {} to {}", display_name, input);
    match handle.set_vcp_feature(INPUT_SELECT, input.value()) {
        Ok(_) => {
            info!("Display {} set to {}", display_name, input);
        }
        Err(err) => {
            error!("Failed to set display {} to {} ({:?})", display_name, input, err);
        }
    }
}

fn displays() -> Vec<Display> {
    let displays = Display::enumerate();
    if !displays.is_empty() {
        return displays;
    }

    // Under some conditions, such as when using a KVM, it's possible for the USB connection/disconnection events to
    // occur before the display(s) become available. We retry once after a bit of a delay in order to be more
    // forgiving with regard to timing.
    let delay_duration = time::Duration::from_millis(RETRY_DELAY_MS);
    warn!(
        "Did not detect any DDC-compatible displays. Retrying after {} second(s)...",
        delay_duration.as_secs()
    );
    thread::sleep(delay_duration);
    Display::enumerate()
}

pub fn log_current_source() {
    let displays = displays();
    if displays.is_empty() {
        error!("Did not detect any DDC-compatible displays!");
        return;
    }
    let unique_names = are_display_names_unique(&displays);
    for (index, mut display) in displays.into_iter().enumerate() {
        let display_name = display_name(&display, if unique_names { None } else { Some(index + 1) });
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(raw_source) => {
                let source = InputSource::from(raw_source.value());
                info!("Display {} is currently set to {}", display_name, source);
            }
            Err(err) => {
                error!("Failed to get current input for display {}: {:?}", display_name, err);
            }
        }
    }
}

pub fn switch(config: &Configuration, switch_direction: SwitchDirection) {
    let displays = displays();
    if displays.is_empty() {
        error!("Did not detect any DDC-compatible displays!");
        return;
    }
    let unique_names = are_display_names_unique(&displays);
    for (index, mut display) in displays.into_iter().enumerate() {
        let display_name = display_name(&display, if unique_names { None } else { Some(index + 1) });
        let input_sources = config.configuration_for_monitor(&display_name);
        debug!("Input sources found for display {}: {:?}", display_name, input_sources);
        if let Some(input) = input_sources.source(switch_direction) {
            try_switch_display(&mut display.handle, &display_name, input);
        } else {
            info!(
                "Display {} is not configured to switch on USB {}",
                display_name, switch_direction
            );
        }
        if let Some(execute_command) = input_sources.execute_command(switch_direction) {
            run_command(execute_command)
        }
    }
    if let Some(execute_command) = config.default_input_sources.execute_command(switch_direction) {
        run_command(execute_command)
    }
}

fn run_command(execute_command: &str) {
    fn try_run_command(execute_command: &str) -> Result<()> {
        let mut arguments = shell_words::split(execute_command)?;
        if arguments.is_empty() {
            return Ok(());
        }

        let executable = arguments.remove(0);
        let output = Command::new(executable).args(arguments).stdin(Stdio::null()).output()?;
        if output.status.success() {
            info!("External command '{}' executed successfully", execute_command);
            Ok(())
        } else {
            let msg = if let Some(code) = output.status.code() {
                format!("Exited with status {}\n", code)
            } else {
                "Exited because of a signal\n".to_string()
            };
            let stdout = if !output.stdout.is_empty() {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    format!("Stdout = [{}]\n", s)
                } else {
                    "Stdout was not UTF-8".to_string()
                }
            } else {
                "No stdout\n".to_string()
            };
            let stderr = if !output.stderr.is_empty() {
                if let Ok(s) = String::from_utf8(output.stderr) {
                    format!("Stderr = [{}]\n", s)
                } else {
                    "Stderr was not UTF-8".to_string()
                }
            } else {
                "No stderr\n".to_string()
            };
            Err(Error::msg(format!("{} {} {}", msg, stdout, stderr)))
        }
    }

    for subcommand in execute_command.split(";") {
        let subcommand = subcommand.trim();
        if !subcommand.is_empty() {
            try_run_command(subcommand).unwrap_or_else(|err| error!("Error executing external command '{}': {}", subcommand, err));
        }
    }
}

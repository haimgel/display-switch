//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::configuration::{Configuration, SwitchDirection};
use crate::input_source::InputSource;
use ddc_hi::{Ddc, Display};

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

fn display_name(display: &Display) -> String {
    format!("'{}'", display.info.id)
}

pub fn log_current_source() {
    let displays = Display::enumerate();
    if displays.is_empty() {
        error!("Did not detect any DDC-compatible displays!");
        return;
    }
    for mut display in displays {
        let display_name = display_name(&display);
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
    let displays = Display::enumerate();
    if displays.is_empty() {
        error!("Did not detect any DDC-compatible displays!");
        return;
    }
    for mut display in displays {
        let display_name = display_name(&display);
        if let Some(input) = config.configuration_for_monitor(&display_name).source(switch_direction) {
            debug!("Setting display {} to {}", display_name, input);
            match display.handle.set_vcp_feature(INPUT_SELECT, input.value()) {
                Ok(_) => {
                    info!("Display {} set to {}", display_name, input);
                }
                Err(err) => {
                    error!("Failed to set display {} to {} ({:?})", display_name, input, err);
                }
            }
        } else {
            info!(
                "Display {} is not configured to switch on USB {}",
                display_name, switch_direction
            );
        }
    }
}

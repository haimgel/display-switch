//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use ddc_hi::{Ddc, Display};
use crate::input_source::InputSource;

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

fn display_name(display: &Display) -> String {
    format!("'{}'", display.info.id)
}

pub fn log_current_source() {
    for mut display in Display::enumerate() {
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

pub fn switch_to(source: InputSource) {
    for mut display in Display::enumerate() {
        let display_name = display_name(&display);
        debug!("Setting display '{}' to {}", display_name, source);
        match display.handle.set_vcp_feature(INPUT_SELECT, source.value()) {
            Ok(_) => {
                info!("Display {} set to {}", display_name, source);
            }
            Err(err) => {
                error!("Failed to set display {} to {} ({:?})", display_name, source, err);
            }
        }
    }
}

//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::configuration::{Configuration, SwitchDirection};
use crate::input_source::InputSource;
use ddc_hi::{Ddc, Display};
use std::collections::HashSet;
use std::{thread, time};

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;
const RETRY_DELAY_MS: u64 = 3000;

fn display_name(display: &Display, index: Option<usize>) -> String {
    let mut str = "'".to_string();
    str += &display.info.id;
    for field in &[&display.info.manufacturer_id,
                   &display.info.model_name,
                   &display.info.serial_number] {
        if let Some(s) = &field {
            str = str + " " + s;
        }
    }
    if let Some(index) = index {
        str = format!("{} #{}", str, index);
    }
    str + "'"
}

fn are_display_names_unique(displays: &[Display]) -> bool {
    let mut hash = HashSet::new();
    displays.iter().all(|display| hash.insert(display_name(display, None)))
}

fn displays() -> Vec<Display> {
    let displays = Display::enumerate();
    if !displays.is_empty() {
        return displays
    }

    // Under some conditions, such as when using a KVM, it's possible for the USB connection/disconnection events to
    // occur before the display(s) become available. We retry once after a bit of a delay in order to be more
    // forgiving with regard to timing.
    let delay_duration = time::Duration::from_millis(RETRY_DELAY_MS);
    warn!("Did not detect any DDC-compatible displays. Retrying after {} second(s)...", delay_duration.as_secs());
    thread::sleep(delay_duration);
    return Display::enumerate();
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

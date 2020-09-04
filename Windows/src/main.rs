//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

#![windows_subsystem = "windows"]
#[macro_use]
extern crate log;

mod configuration;
mod display_control;
mod logging;
mod pnp_detect;
mod usb_devices;

fn main() {
    logging::init_logging().unwrap();
    let config = configuration::Configuration::load().unwrap();
    let mut detector = usb_devices::UsbChangeDetector::new().unwrap();
    let pnp_detect = pnp_detect::PnPDetect::new(move || {
        let changed_devices = detector.detect_changed_devices().unwrap();
        debug!("Detected device change. Added devices: {:?}", changed_devices);
        if changed_devices.added_devices.contains(&config.usb_device) {
            info!("Detected added device we're looking for {:?}", &config.usb_device);
            display_control::wiggle_mouse();
            display_control::switch_to(config.monitor_input_added).unwrap_or_else(|err| {
                error!("Cannot switch monitor input: {:?}", err);
            });
        }
        if changed_devices.removed_devices.contains(&config.usb_device) {
            info!("Detected removed device we're looking for {:?}", &config.usb_device);
            display_control::wiggle_mouse();
            display_control::switch_to(config.monitor_input_removed).unwrap_or_else(|err| {
                error!("Cannot switch monitor input: {:?}", err);
            });
        }
    });
    display_control::log_current_source().unwrap_or_else(|err| {
        error!("Cannot get monitor input: {:?}", err);
    });
    pnp_detect.detect();
}

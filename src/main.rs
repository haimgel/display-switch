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

mod platform;
mod usb_callback;
use platform::PnPDetect;

struct App {
    config: configuration::Configuration,
}

impl usb_callback::UsbCallback for App {
    fn device_added(&self, device_id: &str) {
        debug!("Detected device change. Added device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!(
                "Detected device we're looking for {:?}",
                &self.config.usb_device
            );
            platform::wake_screens();
            display_control::switch_to(self.config.monitor_input).unwrap_or_else(|err| {
                error!("Cannot switch monitor input: {:?}", err);
            });
        }
    }

    fn device_removed(&self, device_id: &str) {
        debug!("Detected device change. Removed device: {:?}", device_id);
    }
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            config: configuration::Configuration::load().unwrap(),
        };
        logging::init_logging().unwrap();
        return app;
    }

    pub fn run(self) {
        display_control::log_current_source();
        let pnp_detector = PnPDetect::new(Box::new(self));
        pnp_detector.detect();
    }
}

fn main() {
    let app = App::new();
    app.run();
}

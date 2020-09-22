//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::configuration::Configuration;
use crate::display_control;
use crate::logging;
use crate::platform::{wake_displays, PnPDetect};
use crate::usb;

pub struct App {
    config: Configuration,
}

impl usb::UsbCallback for App {
    #[allow(unused_must_use)]
    fn device_added(&self, device_id: &str) {
        debug!("Detected device change. Added device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!("Detected device we're looking for {:?}", &self.config.usb_device);
            std::thread::spawn(|| {
                wake_displays().map_err(|err| error!("{:?}", err));
            });
            display_control::switch_to(self.config.monitor_input);
        }
    }

    fn device_removed(&self, device_id: &str) {
        debug!("Detected device change. Removed device: {:?}", device_id);
    }
}

impl App {
    pub fn new() -> Self {
        logging::init_logging().unwrap();
        let config = match Configuration::load() {
            Ok(config) => config,
            Err(err) => {
                error!("Could not load configuration: {:?}", err);
                panic!("Configuration error")
            }
        };
        Self { config }
    }

    pub fn run(self) {
        display_control::log_current_source();
        let pnp_detector = PnPDetect::new(Box::new(self));
        pnp_detector.detect().unwrap();
    }
}

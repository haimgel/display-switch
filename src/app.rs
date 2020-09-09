//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

#![windows_subsystem = "windows"]

use crate::configuration::Configuration;
use crate::display_control;
use crate::logging;
use crate::platform::{PnPDetect, wake_displays};
use crate::usb;

pub struct App {
    config: Configuration,
}

impl usb::UsbCallback for App {
    #[allow(unused_must_use)]
    fn device_added(&self, device_id: &str) {
        debug!("Detected device change. Added device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!(
                "Detected device we're looking for {:?}",
                &self.config.usb_device
            );
            std::thread::spawn(|| {
                wake_displays().map_err(|err| error!("{:?}", err));
            });
            display_control::switch_to(self.config.monitor_input);
            // Apparently, for my monitor (a iiyama PL2779Q) you have to yell
            display_control::switch_to(self.config.monitor_input);
            display_control::switch_to(self.config.monitor_input);
        }    
    }

    #[allow(unused_must_use)]
    fn device_removed(&self, device_id: &str) {
        debug!("Detected device change. Removed device: {:?}", device_id);
        if device_id == self.config.usb_device {
			info!(
				"Detected the device we're looking for {:?}, switching to other input",
				&self.config.usb_device
			);
			std::thread::spawn(|| {
				wake_displays().map_err(|err| error!("{:?}", err));
			});
			display_control::switch_to(self.config.monitor_other);
            // Apparently, for my monitor (a iiyama PL2779Q) you have to yell
			display_control::switch_to(self.config.monitor_other);
			display_control::switch_to(self.config.monitor_other);
		}
    }
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            config: Configuration::load().unwrap(),
        };
        logging::init_logging().unwrap();
        return app;
    }

    pub fn run(self) {
        display_control::log_current_source();
        let pnp_detector = PnPDetect::new(Box::new(self));
        pnp_detector.detect().unwrap();
    }
}


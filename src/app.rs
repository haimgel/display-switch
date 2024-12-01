//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::{Context, Result};

use crate::configuration::{Configuration, SwitchDirection};
use crate::logging;
use crate::platform::{wake_displays, PnPDetect};
use crate::usb;
use crate::{display_control, Args};

pub struct App {
    config: Configuration,
}

impl usb::UsbCallback for App {
    #[allow(unused_must_use)]
    fn device_added(&self, device_id: &str) {
        debug!("Detected device change. Added device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!("Monitored device ({:?}) is connected", &self.config.usb_device);
            std::thread::spawn(|| {
                wake_displays().map_err(|err| error!("{:?}", err));
            });
            display_control::switch(&self.config, SwitchDirection::Connect);
        }
    }

    fn device_removed(&self, device_id: &str) {
        debug!("Detected device change. Removed device: {:?}", device_id);
        if device_id == self.config.usb_device {
            info!("Monitored device is ({:?}) is disconnected", &self.config.usb_device);
            display_control::switch(&self.config, SwitchDirection::Disconnect);
        }
    }
}

impl App {
    pub fn new(args: Args) -> Result<Self> {
        logging::init_logging(args.debug).context("failed to initialize logging")?;
        info!(
            "display-switch v{version} built on {timestamp} from git {git}",
            version = env!("CARGO_PKG_VERSION"),
            timestamp = env!("VERGEN_BUILD_TIMESTAMP"),
            git = env!("VERGEN_GIT_DESCRIBE"),
        );
        let config = Configuration::load().context("failed to load configuration")?;

        Ok(Self { config })
    }

    pub fn run(self) -> Result<()> {
        display_control::log_current_source();
        let pnp_detector = PnPDetect::new(Box::new(self));
        pnp_detector.detect()?;

        Ok(())
    }
}

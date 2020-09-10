//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::usb::{device2str, UsbCallback};
use rusb::{Context, Device, UsbContext};
use anyhow::{anyhow, Result};

/// Detection of plugged in / removed USB devices: uses "libusb" and should work on Linux
/// and MacOS, but not on Windows: libusb does not support hotplug on Windows.
pub struct PnPDetectLibusb {
    callback: Box<dyn UsbCallback>,
}

impl<T: UsbContext> rusb::Hotplug<T> for PnPDetectLibusb {
    fn device_arrived(&mut self, device: Device<T>) {
        device2str(device).map(|str| self.callback.device_added(&str));
    }

    fn device_left(&mut self, device: Device<T>) {
        device2str(device).map(|str| self.callback.device_removed(&str));
    }
}

impl PnPDetectLibusb {
    pub fn new(callback: Box<dyn UsbCallback>) -> Self {
        PnPDetectLibusb { callback }
    }

    pub fn detect(self) -> Result<()> {
        if rusb::has_hotplug() {
            let context = Context::new().unwrap();
            context
                .register_callback(None, None, None, Box::new(self))
                .unwrap();
            loop {
                if let Err(err) = context.handle_events(None) {
                    error!("Error during USB errors handling: {:?}", err)
                };
            };
        } else {
            // This should never happen: hotplug is supported on Linux and MacOS both.
            Err(anyhow!("libusb hotplug api unsupported"))
        }
    }
}

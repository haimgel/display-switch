//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::{anyhow, Result};
use rusb::{Context, Device, HotplugBuilder, Registration, UsbContext};

/// Detection of plugged in / removed USB devices: uses "libusb" and should work on Linux
/// and MacOS, but not on Windows: libusb does not support hotplug on Windows.
pub struct PnPDetectLibusb {
    callback: Box<dyn crate::usb::UsbCallback + Send>,
}

impl<T: UsbContext> rusb::Hotplug<T> for PnPDetectLibusb {
    fn device_arrived(&mut self, device: Device<T>) {
        if let Some(str) = crate::usb::device_id(&device) {
            self.callback.device_added(&str)
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        if let Some(str) = crate::usb::device_id(&device) {
            self.callback.device_removed(&str)
        }
    }
}

impl PnPDetectLibusb {
    pub fn new(callback: Box<dyn crate::usb::UsbCallback + Send>) -> Box<Self> {
        Box::new(PnPDetectLibusb { callback })
    }

    pub fn detect(self) -> Result<()> {
        if rusb::has_hotplug() {
            let context = Context::new()?;

            let _reg: std::option::Option<Registration<rusb::Context>> = Some(
                HotplugBuilder::new()
                    .enumerate(true)
                    .register(&context, Box::new(self))?,
            );

            loop {
                if let Err(err) = context.handle_events(None) {
                    error!("Error during USB errors handling: {:?}", err)
                };
            }
        } else {
            // This should never happen: hotplug is supported on Linux and MacOS both.
            Err(anyhow!("libusb hotplug api unsupported"))
        }
    }
}

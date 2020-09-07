use crate::usb_callback::{device2str, UsbCallback};
use rusb::{Context, Device, UsbContext};

pub struct PnPDetect {
    callback: Box<dyn UsbCallback>,
}

impl<T: UsbContext> rusb::Hotplug<T> for PnPDetect {
    fn device_arrived(&mut self, device: Device<T>) {
        device2str(device).map(|str| self.callback.device_added(&str));
    }

    fn device_left(&mut self, device: Device<T>) {
        device2str(device).map(|str| self.callback.device_removed(&str));
    }
}

impl PnPDetect {
    pub fn new(callback: Box<dyn UsbCallback>) -> Self {
        PnPDetect { callback }
    }

    pub fn detect(self) {
        if rusb::has_hotplug() {
            let context = Context::new().unwrap();
            context
                .register_callback(None, None, None, Box::new(self))
                .unwrap();
            loop {
                context.handle_events(None).unwrap();
            }
        } else {
            panic!("libusb hotplug api unsupported");
        }
    }
}

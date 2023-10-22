//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use rusb::UsbContext;

pub fn device2str<T: UsbContext>(device: rusb::Device<T>) -> Option<String> {
    device
        .device_descriptor()
        .map(|device_desc| format!("{:04x}:{:04x}", device_desc.vendor_id(), device_desc.product_id()))
        .ok()
}

pub trait UsbCallback: Send {
    fn device_added(&self, device_id: &str);
    fn device_removed(&self, device_id: &str);
}

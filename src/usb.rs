//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use rusb::UsbContext;

pub fn device_id<T: UsbContext>(device: &rusb::Device<T>) -> Option<String> {
    device
        .device_descriptor()
        .map(|device_desc| format!("{:04x}:{:04x}", device_desc.vendor_id(), device_desc.product_id()))
        .ok()
}

// pub fn device_product_name<T: UsbContext>(device: &rusb::Device<T>) -> Option<String> {
//     let descriptor = device.device_descriptor().ok()?;
//     let handle = device.open().ok()?;
//     handle.read_product_string_ascii(&descriptor).ok()
// }
//
// pub fn device_manufacturer_name<T: UsbContext>(device: &rusb::Device<T>) -> Option<String> {
//     let descriptor = device.device_descriptor().ok()?;
//     let handle = device.open().ok()?;
//     handle.read_manufacturer_string_ascii(&descriptor).ok()
// }

pub trait UsbCallback: Send {
    fn device_added(&self, device_id: &str);
    fn device_removed(&self, device_id: &str);
}

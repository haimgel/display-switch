//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use std::collections::HashSet;

use anyhow::Result;

pub struct UsbChangeDetector {
    current_devices: HashSet<String>,
}


#[derive(Debug)]
pub struct ChangedDevices {
    pub added_devices: HashSet<String>,
    pub removed_devices: HashSet<String>,
}

impl UsbChangeDetector {
    pub fn new() -> Result<UsbChangeDetector> {
        Ok(UsbChangeDetector {
            current_devices: Self::read_device_list()?,
        })
    }

    // pub fn detect_added_devices(&mut self) -> Result<HashSet<String>> {
    //     let new_devices = Self::read_device_list()?;
    //     let added_devices = &new_devices - &self.current_devices;
    //     self.current_devices = new_devices;
    //     return Ok(added_devices);
    // }

    pub fn detect_changed_devices(&mut self) -> Result<ChangedDevices> {
        let new_devices = Self::read_device_list()?;
        let added_devices = &new_devices - &self.current_devices;
        let removed_devices = &self.current_devices - &new_devices;
        self.current_devices = new_devices;
        return Ok(ChangedDevices {
            added_devices,
            removed_devices,
        });
    }

    fn read_device_list() -> Result<HashSet<String>> {
        Ok(rusb::devices()?
            .iter()
            .map(|device| {
                device.device_descriptor().map(|device_desc| {
                    format!(
                        "{:04x}:{:04x}",
                        device_desc.vendor_id(),
                        device_desc.product_id()
                    )
                })
            })
            .collect::<std::result::Result<_, _>>()?)
    }
}

//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::display_control::DdcControlTrait;
use anyhow::{anyhow, Result};

/// Controls the displays via "DDC.Swift" on MacOS.
pub struct DdcControlMacos();

/// These are exported by the Swift code in mac_ddc/src/mac_ddc.swift
extern "C" {
    fn ddcWriteInputSelect(display_idx: isize, input: u16) -> bool;
    fn ddcReadInputSelect(display_idx: isize) -> isize;
    fn getDisplayCount() -> isize;
}

impl DdcControlTrait for DdcControlMacos {
    fn get_display_range() -> std::ops::Range<isize> {
        unsafe { 0..getDisplayCount() }
    }

    fn ddc_read_input_select(display_idx: isize) -> Result<u16> {
        let source = unsafe { ddcReadInputSelect(display_idx) };
        if source > 0 {
            info!(
                "Display '{:?}' is currently set to 0x{:x}",
                display_idx, source
            );
            Ok(source as u16)
        } else {
            error!("Failed to get current input for display '{:?}'", display_idx);
            Err(anyhow!("DDC error"))
        }
    }

    fn ddc_write_input_select(display_idx: isize, source: u16) -> Result<()> {
        let result = unsafe { ddcWriteInputSelect(display_idx, source) };
        if result {
            info!("Display '{:?}' set to 0x{:x}", display_idx, source);
            Ok(())
        } else {
            error!("Failed to set display '{:?}' to 0x{:x}", display_idx, source);
            Err(anyhow!("DDC error"))
        }
    }
}

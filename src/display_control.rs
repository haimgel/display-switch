//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use std::thread::sleep;

use anyhow::Result;
use ddc::Ddc;
use ddc_winapi::Monitor;
use serde::Deserialize;
use winapi::_core::time::Duration;
use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum InputSource {
    DisplayPort1 = 0x0f,
    DisplayPort2 = 0x10,
    Hdmi1 = 0x11,
    Hdmi2 = 0x12,
}

pub fn log_current_source() -> Result<()> {
    for mut ddc in Monitor::enumerate()? {
        let source = ddc.get_vcp_feature(INPUT_SELECT)?.value();
        info!("Monitor '{:?}' is currently set to 0x{:x}", ddc, source);
    }
    Ok(())
}

pub fn switch_to(source: InputSource) -> Result<()> {
    for mut ddc in Monitor::enumerate()? {
        info!("Setting monitor '{:?}' to {:?}", ddc, source);
        ddc.set_vcp_feature(INPUT_SELECT, source as u16)?;
    }
    Ok(())
}

// Move a mouse a little bit, this causes the displays to wake up
pub fn wiggle_mouse() {
    unsafe {
        mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
        sleep(Duration::from_millis(50));
        mouse_event(MOUSEEVENTF_MOVE, 0, 0xffffffff, 0, 0);
    }
}

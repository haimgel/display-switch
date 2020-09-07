//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::platform::DDCControlImpl;
use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum InputSource {
    DisplayPort1 = 0x0f,
    DisplayPort2 = 0x10,
    Hdmi1 = 0x11,
    Hdmi2 = 0x12,
}

pub trait DDCControl {
    fn get_display_range() -> std::ops::Range<isize>;
    fn ddc_read_input_select(screen_idx: isize) -> Result<u16>;
    fn ddc_write_input_select(screen_idx: isize, value: u16) -> Result<()>;
}

#[allow(unused_must_use)]
pub fn log_current_source() {
    for display in DDCControlImpl::get_display_range() {
        DDCControlImpl::ddc_read_input_select(display);
    }
}

#[allow(unused_must_use)]
pub fn switch_to(source: InputSource) {
    for display in DDCControlImpl::get_display_range() {
        DDCControlImpl::ddc_write_input_select(display, source as u16);
    }
}

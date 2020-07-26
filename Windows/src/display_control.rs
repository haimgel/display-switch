// use crate::InputSource::DisplayPort1;
use ddc::Ddc;
use ddc_winapi::Monitor;
use serde::Deserialize;

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum InputSource {
    DisplayPort1 = 0x0f,
    DisplayPort2 = 0x10,
    Hdmi1 = 0x11,
    Hdmi2 = 0x12,
}

pub fn switch_to(source: InputSource) {
    for mut ddc in Monitor::enumerate().unwrap() {
        ddc.set_vcp_feature(INPUT_SELECT, source as u16).unwrap();
    }
}


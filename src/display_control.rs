//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use crate::platform::DdcControl;
use crate::input_source::InputSource;
use anyhow::Result;

/// The subset of DDC that we need for display control. Need to have this indirection because
/// there's no MacOS support in ddc-hi and we want to some uniformity in the way how we control
/// the displays.
pub trait DdcControlTrait {
    fn get_display_range() -> std::ops::Range<isize>;
    fn ddc_read_input_select(display_idx: isize) -> Result<u16>;
    fn ddc_write_input_select(display_idx: isize, value: u16) -> Result<()>;
}

#[allow(unused_must_use)]
pub fn log_current_source() {
    for display in DdcControl::get_display_range() {
        DdcControl::ddc_read_input_select(display);
    }
}

#[allow(unused_must_use)]
pub fn switch_to(source: InputSource) {
    for display in DdcControl::get_display_range() {
        DdcControl::ddc_write_input_select(display, source.value());
    }
}

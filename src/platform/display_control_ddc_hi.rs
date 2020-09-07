use crate::display_control::DDCControl;
use anyhow::Result;
use ddc_hi::{Ddc, Display};

pub struct DDCControlDdcHi();

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

impl DDCControlDdcHi {
    fn ddc_for(screen_idx: isize) -> Option<&Display> {
        Display::enumerate().get(screen_idx)
    }
}

impl DDCControl for DDCControlDdcHi {
    fn get_display_range() -> std::ops::Range<isize> {
        0 .. Display::enumerate().len()
    }

    fn ddc_read_input_select(screen_idx: isize) -> Result<u16> {
        let display = ddc_for(screen_idx)?;
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(source) => {
                info!(
                    "Monitor '{:?}' is currently set to 0x{:x}",
                    display.info, source
                );
                Ok(source)
            }
            Err(err) => {
                error!(
                    "Failed to get current input for monitor '{:?}': {:?}",
                    display.info, err
                );
                Err(err)
            }
        }
    }

    fn ddc_write_input_select(screen_idx: isize, source: u16) -> Result<()> {
        let display = ddc_for(screen_idx)?;
        info!("Setting monitor '{:?}' to 0x{:x}", display.info, source);
        match display.handle.set_vcp_feature(INPUT_SELECT, source) {
            Ok(_) => {
                info!("Monitor '{:?}' set to 0x{:x}", display.info, source);
                Ok(())
            }
            Err(err) => {
                error!(
                    "Failed to set monitor '{:?}' to 0x{:x} ({:?})",
                    display.info, source, err
                );
                Err(err)
            }
        }
    }
}

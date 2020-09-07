use crate::display_control::DdcControlTrait;
use anyhow::{anyhow, Result};
use ddc_hi::{Ddc, Display};

/// Controls the displays via "ddc-hi" crate: should support Windows and Linux.
pub struct DdcControlDdcHi();

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

fn ddc_for(display_idx: isize) -> Result<Display> {
    let mut displays = Display::enumerate();
    if (display_idx >= 0) && ((display_idx as usize) < displays.len()) {
        Ok(displays.remove(display_idx as usize))
    } else {
        Err(anyhow!("Display not found"))
    }
}

fn display_name(display: &Display, display_idx: isize) -> String {
    // TODO: Verify that formatting here makes sense on Linux as well
    format!("'{}' #{}", display.info.id, display_idx)
}

impl DdcControlTrait for DdcControlDdcHi {
    fn get_display_range() -> std::ops::Range<isize> {
        0..Display::enumerate().len() as isize
    }

    fn ddc_read_input_select(display_idx: isize) -> Result<u16> {
        let mut display = ddc_for(display_idx)?;
        let display_name = display_name(&display, display_idx);
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(source) => {
                info!(
                    "Display {} is currently set to 0x{:x}",
                    display_name,
                    source.value()
                );
                Ok(source.value())
            }
            Err(err) => {
                error!(
                    "Failed to get current input for display {}: {:?}",
                    display_name, err
                );
                Err(anyhow!(err))
            }
        }
    }

    fn ddc_write_input_select(display_idx: isize, source: u16) -> Result<()> {
        let mut display = ddc_for(display_idx)?;
        let display_name = display_name(&display, display_idx);
        debug!("Setting display '{}' to 0x{:x}", display_name, source);
        match display.handle.set_vcp_feature(INPUT_SELECT, source) {
            Ok(_) => {
                info!("Display {} set to 0x{:x}", display_name, source);
                Ok(())
            }
            Err(err) => {
                error!(
                    "Failed to set display {} to 0x{:x} ({:?})",
                    display_name, source, err
                );
                Err(anyhow!(err))
            }
        }
    }
}

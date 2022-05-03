//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//
use anyhow::Result;

#[cfg(target_os = "windows")]
/// Move a mouse a little bit, this causes the displays to wake up
pub fn wake_displays() -> Result<()> {
    use std::{thread, time};
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

    unsafe {
        mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
        thread::sleep(time::Duration::from_millis(50));
        mouse_event(MOUSEEVENTF_MOVE, 0, 0xffffffff, 0, 0);
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn wake_displays() -> Result<()> {
    use anyhow::anyhow;
    use std::process::Command;

    match Command::new("/usr/bin/caffeinate").args(&["-u", "-t", "10"]).status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(anyhow!(
                    "Couldn't wake displays, 'caffeinate' returned {:?}",
                    status.code()
                ))
            }
        }
        Err(err) => Err(anyhow!("Couldn't wake displays, couldn't run 'caffeinate': {}", err)),
    }
}

#[cfg(target_os = "linux")]
pub fn wake_displays() -> Result<()> {
    use anyhow::Context;
    use std::{thread, time};
    use uinput::{Device, event::keyboard};

    fn make_kbd_device() -> Result<Device> {
        Ok(uinput::default()?
            .name("display-switch")?
            .event(uinput::event::Keyboard::All)?
            .create()?)
    }

    let mut device = make_kbd_device().context("Couldn't wake displays: couldn't configure uinput")?;

    // This sleep appears to be necessary based on testing.
    // Possibly X does not immediately recognize the new device?
    thread::sleep(time::Duration::from_secs(1));

    device.click(&keyboard::Key::RightAlt)?;
    device.synchronize()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wake_displays() {
        assert!(wake_displays().is_ok());
    }
}

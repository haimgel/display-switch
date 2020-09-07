//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

#[cfg(target_os = "windows")]
/// Move a mouse a little bit, this causes the displays to wake up
pub fn wake_displays() {
    use std::{thread, time};
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

    unsafe {
        mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
        thread::sleep(time::Duration::from_millis(50));
        mouse_event(MOUSEEVENTF_MOVE, 0, 0xffffffff, 0, 0);
    }
}

#[cfg(target_os = "macos")]
pub fn wake_displays() {
    use std::process::Command;
    match Command::new("/usr/bin/caffeinate").args(&["-u", "-t", "10"]).status() {
        Ok(status) => {
            if status.success() {
                info!("Woke all displays for 10 seconds")
            } else {
                error!("Couldn't wake displays, 'caffeinate' returned {:?}", status.code())
            }
        },
        Err(err) => {
            error!("Couldn't wake displays, couldn't run 'caffeinate': {}", err)
        }
    }
}

#[cfg(target_os = "linux")]
pub fn wake_displays() {
    // TODO
}

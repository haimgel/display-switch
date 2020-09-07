#[cfg(target_os = "windows")]
/// Move a mouse a little bit, this causes the displays to wake up
pub fn wake_screens() {
    use std::{thread, time};
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

    unsafe {
        mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
        thread::sleep(time::Duration::from_millis(50));
        mouse_event(MOUSEEVENTF_MOVE, 0, 0xffffffff, 0, 0);
    }
}

#[cfg(target_os = "macos")]
pub fn wake_screens() {
    // TODO
}

#[cfg(target_os = "linux")]
pub fn wake_screens() {
    // TODO
}

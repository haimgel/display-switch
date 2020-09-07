#[cfg(target_os = "macos")]
mod pnp_detect_libusb;
#[cfg(target_os = "macos")]
pub use pnp_detect_libusb::PnPDetectLibusb as PnPDetect;
#[cfg(target_os = "macos")]
mod display_control_macos;
#[cfg(target_os = "macos")]
pub use display_control_macos::DdcControlMacos as DdcControl;

#[cfg(target_os = "linux")]
mod pnp_detect_libusb;
#[cfg(target_os = "linux")]
pub use pnp_detect_libusb::PnPDetectLibusb as PnPDetect;
#[cfg(target_os = "linux")]
mod display_control_ddc_hi;
#[cfg(target_os = "linux")]
pub use display_control_ddc_hi::DdcControlDdcHi as DdcControl;

#[cfg(target_os = "windows")]
mod pnp_detect_windows;
#[cfg(target_os = "windows")]
pub use pnp_detect_windows::PnPDetectWindows as PnPDetect;
#[cfg(target_os = "windows")]
mod display_control_ddc_hi;
#[cfg(target_os = "windows")]
pub use display_control_ddc_hi::DdcControlDdcHi as DdcControl;

mod wake_screens;
pub use wake_screens::wake_screens;

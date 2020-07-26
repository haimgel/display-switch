mod pnp_detect;
mod display_control;
mod usb_devices;
mod configuration;

fn main() {
    let config = configuration::Configuration::load().unwrap();
    let mut detector = usb_devices::UsbChangeDetector::new().unwrap();
    print!("Starting up!\n");
    let pnp_detect = pnp_detect::PnPDetect::new(move || {
        let added_devices = detector.detect_added_devices().unwrap();
        if added_devices.contains(&config.usb_device) {
            print!("Detected addition if device we're looking for");
            display_control::switch_to(config.monitor_input);
        }
    });
    pnp_detect.detect();
}

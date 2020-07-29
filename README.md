# Turn a $30 USB switch into a full-featured KVM

This utility watches for USB device connect/disconnect events and switches monitor inputs via DDC/CI. This turns
a simple USB switch into a full-fledged KVM solution: press one button on your USB switch and all your monitors
connect to a different input.

It is supposed to be installed on all computers that could be connected to these monitors, since the app only switches
monitors "one way" and relies on itself running on the other computers to switch it "the other way" as needed.
 
## Platforms supported

The app is written twice: once for MacOS (in Swift) and once for Windows (in Rust). The idea is dead simple, but I
haven't found good cross-platform support for DDC, USB, device plug-n-play and power management in any one language
except C, and I didn't want to write this in C.

## Configuration

The configuration is pretty similar on both platforms:

On MacOS: the configuration file is expected in `~/Library/Preferences/display-switch.ini`
On Windows: the configuration file is expected in `/Users/USERNAME/AppData/Roaming/display-switch/display-switch.ini`

Configuration file settings:

```ini
  usb_device = "1050:0407"
  monitor_input = "Hdmi1"
```

`usb_device` is which USB device to watch (vendor id / device id in hex), and `monitor_input` is which monitor input
to switch to, when this device is connected. Supported values are `Hdmi1`, `Hdmi2`, `DisplaySwitch1`, `DisplaySwitch2`
If your monitor has an USB-C port, it's usually reported as `DisplaySwitch2`

## Logging

On MacOS: the log file is written to the system log.
On WindowS: the log file is written to `/Users/USERNAME/AppData/Local/display-switch/display-switch.log`

## Building from source, running on startup

These exercises are left to the reader for now :-)

[![build](https://github.com/haimgel/display-switch/workflows/build/badge.svg?branch=master)](https://github.com/haimgel/display-switch/actions)
[![GitHub license](https://img.shields.io/github/license/haimgel/display-switch)](https://github.com/haimgel/display-switch/blob/master/LICENSE)

# Turn a $30 USB switch into a full-featured KVM

This utility watches for USB device connect/disconnect events and switches monitor inputs via DDC/CI. This turns
a simple USB switch into a full-fledged KVM solution: press one button on your USB switch and all your monitors
connect to a different input.

It is supposed to be installed on all computers that could be connected to these monitors, since the app only switches
monitors "one way" and relies on itself running on the other computers to switch it "the other way" as needed.
 
## Platforms supported

The app should function on MacOS and Windows. Linux support is planned in a future release. Most of the code is in
Rust, with the exception of DDC support on MacOS, which is done via statically-linked Swift library.

## Configuration

The configuration is pretty similar on all platforms:

On MacOS: the configuration file is expected in `~/Library/Preferences/display-switch.ini`
On Windows: the configuration file is expected in `/Users/USERNAME/AppData/Roaming/display-switch/display-switch.ini`

Configuration file settings:

```ini
  usb_device = "1050:0407"
  monitor_input = "Hdmi1"
```

`usb_device` is which USB device to watch (vendor id / device id in hex), and `monitor_input` is which monitor input
to switch to, when this device is connected. Supported values are `Hdmi1`, `Hdmi2`, `DisplayPort1`, `DisplayPort2`
If your monitor has an USB-C port, it's usually reported as `DisplayPort2`. Input can also be specified as a "raw"
decimal or hexadecimal value: `monitor_input = 0x10`

### USB Device IDs
To locate the ID of your USB device ID on Windows:
1. Open Device Manager
2. Locate the USB device, view the properties
3. Switch to the *Details* tab and select *Hardware IDs* in the Property dropdown
4. You should see a value similar to `HID\VID_046D&PID_C52B&MI_00` (the exact values will differ) - the USB device ID is a combination of the *Vendor ID* and the *Product ID* - for example, in this case it would be `046D:C52B`

To locate the ID of your USB device ID on MacOS, open a terminal and run the following:
```bash
brew install lsusb

$ lsusb > a
$ lsusb > b
$ opendiff a b
```
In the command output, the highlighted lines show you which USB IDs are most relevant.

## Logging

* On MacOS: the log file is written to `/Users/USERNAME/Library/Logs/display-switch/display-switch.log`
* On WindowS: the log file is written to `/Users/USERNAME/AppData/Local/display-switch/display-switch.log`

## Building from source

### Windows

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release`

### MacOS

[Install Xcode](https://developer.apple.com/xcode/), [install Rust](https://www.rust-lang.org/tools/install), then do
`cargo build --release` 

## Running on startup

### Windows

Copy `display_switch.exe` from `target\release` (where it was built in the previous step) to 
`C:\Users\Username\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup` (replace Username with your 
Windows user name).

### MacOS

```bash
  # Get your INI file in order! (see above)
  cp target/release/display_switch /usr/local/bin
  cp dev.haim.display-switch.daemon.plist ~/Library/LaunchAgents/
  launchctl load ~/Library/LaunchAgents/dev.haim.display-switch.daemon.plist
```

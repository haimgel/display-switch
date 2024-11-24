[![build](https://github.com/haimgel/display-switch/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/haimgel/display-switch/actions)
[![GitHub license](https://img.shields.io/github/license/haimgel/display-switch)](https://github.com/haimgel/display-switch/blob/main/LICENSE)

# Turn a $30 USB switch into a full-featured KVM

This utility watches for USB device connect/disconnect events and switches monitor inputs via DDC/CI. This turns
a simple USB switch into a full-fledged KVM solution: press one button on your USB switch and all your monitors
connect to a different input.

It is supposed to be installed on all computers that could be connected to these monitors, since the app only switches
monitors "one way" and relies on itself running on the other computers to switch it "the other way" as needed.
 
## Platforms supported

The app should function on MacOS, Windows, and Linux.

## Installation
 * Linux and Windows: download and extract the files from the releases page and place them where
   you see fit.
 * MacOS: `display_switch` can be installed with Homebrew:
   ```bash
   brew install haimgel/tools/display_switch
   ```

## Configuration

The configuration is pretty similar on all platforms:

On MacOS: the configuration file is expected in `~/Library/Preferences/display-switch.ini`
On Windows: the configuration file is expected in `%APPDATA%\display-switch\display-switch.ini`
On Linux: the configuration file is expected in `$XDG_CONFIG_HOME/display-switch/display-switch.ini` or `~/.config/display-switch/display-switch.ini`

Configuration file settings:

```ini
  usb_device = "1050:0407"
  on_usb_connect = "Hdmi1"
  on_usb_disconnect = "Hdmi2"
```

`usb_device` is which USB device to watch (vendor id / device id in hex), and `on_usb_connect` is which monitor input
to switch to, when this device is connected. Supported values are `Hdmi1`, `Hdmi2`, `DisplayPort1`, `DisplayPort2`, `Dvi1`, `Dvi2`, `Vga1`.
If your monitor has an USB-C port, it's usually reported as `DisplayPort2`. Input can also be specified as a "raw"
decimal or hexadecimal value: `on_usb_connect = 0x10`

The optional `on_usb_disconnect` settings allows to switch in the other direction when the USB device is disconnected.
Note that the preferred way is to have this app installed on both computers. Switching "away" is problematic: if the
other computer has put the monitors to sleep, they will switch immediately back to the original input.

### Different inputs on different monitors
`display-switch` supports per-monitor configuration: add one or more monitor-specific configuration sections to set
monitor-specific inputs. For example:

```ini
on_usb_connect = "DisplayPort2"
on_usb_disconnect = "Hdmi1"

[monitor1]
monitor_id = "len"
on_usb_connect = "DisplayPort1"

[monitor2]
monitor_id = "dell"
on_usb_connect = "hdmi2"
```

`monitor_id` specifies a case-insensitive substring to match against the monitor ID. For example, 'len' would match
`LEN P27u-10 S/N 1144206897` monitor ID. If more than one section has a match, a first one will be used.
`on_usb_connect` and `on_usb_disconnect`, if defined, take precedence over global defaults.

_Tips for Windows_: monitors can be renamed in the Registry at
`\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Enum\DISPLAY\{MODEL_ID}\{CONNECTION_ID}`. Edit the `DeviceDesc` value and change the name after the last semicolon. This is especially helpful in case they are all just "Generic PnP Monitor".

### Running external commands
`display-switch` supports running external commands upon connection or disconnection of USB devices. This configuration
can be global (runs every time a configured USB device is connected or disconnected) or per-monitor (runs only when
a given monitor is being switched):

```ini
usb_device = "1050:0407"
on_usb_connect = "Hdmi1"
on_usb_disconnect = "DisplayPort2"
on_usb_connect_execute = "echo connected"
on_usb_disconnect_execute = "echo disconnected"

[monitor1]
monitor_id="foobar"
on_usb_connect_execute = "echo usb connected, monitor 'foobar' being switched"
on_usb_disconnect_execute = "'c:\\program files\\my app.exe' --parameter"
```

Notes: 
1. External applications are executed as the same user that started `display-switch`. 
2. This program supports splitting supplied configuration into application name and parameters, but no other shell features are supported.
3. If the application path contains spaces, surround the full file path with single quotes.
4. On Windows, escape the backslashes (replace \ with \\, see the example above).

### USB Device IDs

#### Windows
To locate the ID of your USB device ID on Windows:
1. Open Device Manager
2. Locate the USB device, view the properties
3. Switch to the *Details* tab and select *Hardware IDs* in the Property dropdown
4. You should see a value similar to `HID\VID_046D&PID_C52B&MI_00` (the exact values will differ) - the USB device ID is a combination of the *Vendor ID* and the *Product ID* - for example, in this case it would be `046D:C52B`

#### MacOS
To locate the ID of your USB device ID on MacOS, open a terminal and run the following:
```bash
brew install lsusb

$ lsusb > a
<switch the usb dock here>
$ lsusb > b
$ opendiff a b
```
In the command output, the highlighted lines show you which USB IDs are most relevant.


For a full list of USB devices:
```
system_profiler SPUSBDataType
```
**Important**: The format for your display-switch.ini is VendorID:ProductID. VendorID is displyed *second* in the `system_profiler` output

#### Linux
Requires additional packages, install via: `sudo apt install libxi-dev xorg-dev`

To locate the ID of your USB device on Linux, first install `lsusb`, which your Linux
distro should have a package for. (On Debian, Ubuntu and RedHat, the package name is `usbutils`.)
Then, in a terminal, run the following:
```
$ lsusb > a
<switch the usb dock here>
$ lsusb > b
$ diff -u a b
```
The diff output will show which USB IDs are most relevant.

## Logging

* On MacOS: the log file is written to `/Users/USERNAME/Library/Logs/display-switch/display-switch.log`
* On Windows: the log file is written to `%LOCALAPPDATA%\display-switch\display-switch.log`
* On Linux: The log file is written to `$XDG_DATA_HOME/display-switch/display-switch.log`
 or `~/.local/share/display-switch/display-switch.log`

## Building from source

### Windows

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release`

### MacOS

[Install Xcode](https://developer.apple.com/xcode/), [install Rust](https://www.rust-lang.org/tools/install), then do
`cargo build --release` 

### Linux

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release`

## Running on startup

### Windows

Copy `display_switch.exe` from `target\release` (where it was built in the previous step) to 
`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`.

### MacOS

```bash
  # Get your INI file in order! (see above)
  cp target/release/display_switch /usr/local/bin/
  cp dev.haim.display-switch.daemon.plist ~/Library/LaunchAgents/
  launchctl load ~/Library/LaunchAgents/dev.haim.display-switch.daemon.plist
```
### Linux
Copy built executable:

```bash
  cp target/release/display_switch /usr/local/bin/
```
Enable read/write access to i2c devices for users in `i2c` group. Run as root :

```bash
groupadd i2c
echo 'KERNEL=="i2c-[0-9]*", GROUP="i2c"' >> /etc/udev/rules.d/10-local_i2c_group.rules
udevadm control --reload-rules && udevadm trigger
```

Then add your user to the i2c group :

```
sudo usermod -aG i2c $(whoami)
```

Create a systemd unit file in your user directory (`/home/$USER/.config/systemd/user/display-switch.service`) with contents

```
[Unit]
Description=Display switch via USB switch

[Service]
ExecStart=/usr/local/bin/display_switch
Type=simple
StandardOutput=journal
Restart=always

[Install]
WantedBy=default.target
```

Create the config file at `/home/$USER/.config/display-switch/display-switch.ini`.
Then enable the service with

```bash
systemctl --user daemon-reload
systemctl --user enable display-switch.service
systemctl --user start display-switch.service
```

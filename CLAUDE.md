# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust application that converts a simple USB switch into a KVM (Keyboard, Video, Mouse) solution by watching for USB device connect/disconnect events and automatically switching monitor inputs via DDC/CI commands. The app runs on all computers connected to shared monitors and coordinates input switching.

## Core Architecture

### Main Components

- **`main.rs`** - Entry point with CLI argument parsing using clap
- **`app.rs`** - Main application logic implementing `UsbCallback` trait for USB events
- **`configuration.rs`** - INI file configuration parsing with per-monitor support using serde
- **`display_control.rs`** - DDC/CI monitor control using `ddc-hi` crate
- **`usb.rs`** - USB device monitoring abstraction
- **`input_source.rs`** - Monitor input source definitions (HDMI, DisplayPort, etc.)
- **`platform/`** - Platform-specific implementations:
  - `pnp_detect_libusb.rs` - USB detection for macOS/Linux using libusb
  - `pnp_detect_windows.rs` - USB detection for Windows using WinAPI
  - `wake_displays.rs` - Platform-specific display wake functionality

### Flow

1. App loads configuration from platform-specific INI file location
2. Starts USB device monitoring using platform-specific PnP detection
3. On USB connect/disconnect events matching configured device ID:
   - Enumerates DDC-compatible displays
   - Switches each display to configured input source
   - Optionally executes configured external commands

### Platform Support

Cross-platform with platform-specific dependencies:
- **macOS**: Uses `ddc-macos` for display control
- **Linux**: Uses `ddc-i2c` and requires i2c device permissions
- **Windows**: Uses `ddc-winapi` and `nvapi` for display control

## Development Commands

### Building
```bash
# Debug build (creates universal binary on macOS)
make build-debug

# Release build (creates universal binary on macOS)
make build-release

# Standard cargo build (non-macOS or simple builds)
cargo build --release
```

### Testing
```bash
make test
# or
cargo test
```

### Running
```bash
# Run with debug logging
./target/release/display_switch --debug

# Check version
./target/release/display_switch --version
```

## Configuration

The app expects an INI configuration file at:
- **macOS**: `~/Library/Preferences/display-switch.ini`
- **Windows**: `%APPDATA%\display-switch\display-switch.ini`
- **Linux**: `$XDG_CONFIG_HOME/display-switch/display-switch.ini` or `~/.config/display-switch/display-switch.ini`

Configuration supports:
- Global USB device monitoring and default input switching
- Per-monitor configuration with monitor ID matching
- External command execution on connect/disconnect events

## Key Dependencies

- **config** - INI file parsing
- **ddc/ddc-hi** - Cross-platform DDC/CI monitor control
- **rusb** - USB device monitoring
- **serde** - Configuration deserialization
- **anyhow** - Error handling
- **clap** - CLI argument parsing
- **simplelog** - Logging to platform-specific log files

## Testing Strategy

The project uses standard Rust unit tests with `cargo test`. Tests cover:
- Configuration parsing and deserialization
- Per-monitor configuration matching
- Input source value conversion

## Build System

Uses a Makefile wrapper around Cargo that:
- On macOS: Creates universal binaries supporting both Intel and ARM architectures
- On other platforms: Uses standard cargo commands
- Includes packaging targets for release distribution
//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

import Foundation
import AppKit
import DDC

class App: USBWatcherDelegate {
    let config: Config
    var mainLoopRunning = false
    var usbWatcher: USBWatcher!
    var waker: Process!

    // Wake the screens when switching, otherwise the screens will auto-switch back (if the input we're switching to
    // is not active at the moment. There is probably a smarter way to do this,
    // but "caffeinate" works and is documented.
    func wakeDisplays() {
        waker = Process()
        waker.executableURL = URL(fileURLWithPath: "/usr/bin/caffeinate")
        waker.arguments = ["-u", "-t 10"]
        if (try? waker.run()) != nil {
            logger.infoMessage("Waking the screens for 10 seconds")
        } else {
            logger.errorMessage("Couldn't run waker")
        }
    }

    // Use DDC to switch all attached external monitors to a configured input
    func switchDisplays() {
        logger.infoMessage("Switching displays to \(config.monitorInput)")
        for screen in NSScreen.screens {
            guard let ddc = DDC(for: screen) else { continue }
            let currentValue = ddc.read(command: .inputSelect)
            let newValue = config.monitorInput.ddcValue()
            if ddc.write(command: .inputSelect, value: newValue) {
                logger.infoMessage("Switched \(screen) from \(currentValue) to \(newValue)")
            } else {
                logger.errorMessage("Couldn't switch \(screen) from \(currentValue) to \(newValue)")
            }
        }
    }

    // MARK: USBWatcherDelegate

    func deviceAdded(_ device: io_object_t) {
        guard mainLoopRunning, let deviceId = device.idString(), let deviceName = device.name() else { return }
        logger.debugMessage("Device '\(deviceId)' ('\(deviceName)') has been added")
        if deviceName.contains(config.usbDevice) || deviceId == config.usbDevice {
            wakeDisplays()
            switchDisplays()
        }
    }

    func deviceRemoved(_ device: io_object_t) {
        guard let deviceName = device.name() else { return }
        logger.debugMessage("Device '\(deviceName)' has been removed")
    }

    func logConnectedDevices() {
        for screen in NSScreen.screens {
            guard DDC(for: screen) != nil else { continue }
            logger.debugMessage("Detected screen '\(screen)'")
        }
    }

    init() throws {
        logger.infoMessage("Initializing")
        config = try Config.load()
        usbWatcher = USBWatcher(delegate: self)
        logConnectedDevices()
    }

    func run() {
        mainLoopRunning = true
        logger.debugMessage("Starting the main loop")
        CFRunLoopRun()
    }
}

let app = try App()
app.run()

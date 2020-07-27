//
//  main.swift
//  DisplaySwitch
//
//  Created by Haim Gelfenbeyn on 2020-07-10.
//  Copyright © 2020 Haim Gelfenbeyn. All rights reserved.
//

import Foundation
import AppKit
import DDC

class App: USBWatcherDelegate {
    let config: Config
    var mainLoopRunning = false
    var usbWatcher: USBWatcher!

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

    func deviceAdded(_ device: io_object_t) {
        guard mainLoopRunning, let deviceId = device.idString(), let deviceName = device.name() else { return }
        logger.debugMessage("Device '\(deviceId)' ('\(deviceName)') has been added")
        if deviceName.contains(config.usbDevice) || deviceId == config.usbDevice {
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

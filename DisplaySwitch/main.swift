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
    var usbWatcher: USBWatcher!

    func switchDisplays() {
        logger.infoMessage("Switching displays to \(config.monitorInput)")
    }

    func deviceAdded(_ device: io_object_t) {
        guard let deviceName = device.name() else { return }
        logger.debugMessage("Device '\(deviceName)' has been added")
        if let name = device.name(), name.contains(config.usbDevice) {
            switchDisplays()
        }
    }

    func deviceRemoved(_ device: io_object_t) {
        guard let deviceName = device.name() else { return }
        logger.debugMessage("Device '\(deviceName)' has been removed")
    }

    func logConnectedDevices() {
        for screen in NSScreen.screens {
            guard let ddc = DDC(for: screen) else { continue }
            logger.debugMessage("Detected screen \(ddc.edid())")
        }
    }

    init() throws {
        logger.infoMessage("Starting up")
        config = try Config.load()
        usbWatcher = USBWatcher(delegate: self)
    }

    func run() {
        logConnectedDevices()
        CFRunLoopRun()
    }
}

let app = try App()
app.run()

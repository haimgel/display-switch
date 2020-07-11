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

print("Hello, World!")
var example = usbDelegate()
CFRunLoopRun()

/*
for screen in NSScreen.screens{
    debugPrint("========================================")
    DDC(for: screen)?.write(command: .inputSelect, value: 16)
}
*/
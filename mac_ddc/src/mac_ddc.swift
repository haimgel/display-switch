//
// mac_ddc.swift
//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

import Foundation
import AppKit
import DDC

func ddc(for screenIdx: Int) -> DDC? {
    let screens = NSScreen.screens
    guard screenIdx >= 0, screenIdx < screens.count else { return nil }
    return DDC(for: screens[screenIdx])
}

/// Use DDC to write inputSelect command
@_cdecl("ddcWriteInputSelect")
public func ddcWriteInputSelect(screenIdx: Int, input: UInt16) -> Bool {
    guard let ddc = ddc(for: screenIdx) else { return false }
    return ddc.write(command: .inputSelect, value: input)
}

/// Use DDC to read inputSelect command.
/// -1 represents Nil here: the actual return type is UInt16
@_cdecl("ddcReadInputSelect")
public func ddcReadInputSelect(screenIdx: Int) -> Int {
    guard let ddc = ddc(for: screenIdx) else { return -1 }
    guard let (currentValue, _) = ddc.read(command: .inputSelect) else { return -1 }
    return Int(currentValue)
}

@_cdecl("getDisplayCount")
public func getDisplayCount() -> Int {
    return NSScreen.screens.count
}

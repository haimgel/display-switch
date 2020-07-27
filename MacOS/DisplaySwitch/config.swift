//
// Created by Haim Gelfenbeyn on 2020-07-10.
// Copyright (c) 2020 Hagel Technologies Ltd. All rights reserved.
//

import Foundation
import TOMLDecoder

// This descends from String and not Int16 just to use the symbolic names in the config
enum InputSource: String, Codable {
    case DisplayPort1
    case DisplayPort2
    case Hdmi1
    case Hdmi2

    func ddcValue() -> UInt16 {
        switch self {
        case .DisplayPort1: return 0x0f
        case .DisplayPort2: return 0x10
        case .Hdmi1: return 0x11
        case .Hdmi2: return 0x12
        }
    }
}

struct Config: Codable {
    let usbDevice: String
    let monitorInput: InputSource

    static func load() throws -> Config {
        let configPath = FileManager()
                .homeDirectoryForCurrentUser
                .appendingPathComponent("Library/Preferences", isDirectory: true)
                .appendingPathComponent("display-switch.ini")
        let configStr = try String(contentsOf: configPath, encoding: .utf8)
        let decoder = TOMLDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        return try decoder.decode(Config.self, from: configStr)
    }
}

//
// Created by Haim Gelfenbeyn on 2020-07-10.
// Copyright (c) 2020 Hagel Technologies Ltd. All rights reserved.
//

import Foundation
import TOMLDecoder

struct Config: Codable {
    let usbDevice: String
    let monitorInput: Int

    static func load() throws -> Config {
        let configPath = FileManager()
                .homeDirectoryForCurrentUser
                .appendingPathComponent(".displaySwitch")
        let configStr = try String(contentsOf: configPath, encoding: .utf8)
        return try TOMLDecoder().decode(Config.self, from: configStr)
    }
}

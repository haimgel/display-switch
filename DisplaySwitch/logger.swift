//
// Created by Haim Gelfenbeyn on 2020-07-10.
// Copyright (c) 2020 Hagel Technologies Ltd. All rights reserved.
//

import Foundation
import Willow

public let logger = Logger(
        logLevels: [.all],
        writers: [
            ConsoleWriter(),
            OSLogWriter(subsystem: "com.hageltech.DisplaySwitch", category: "utility")
        ])

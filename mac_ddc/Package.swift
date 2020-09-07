// swift-tools-version:5.1
import PackageDescription

let package = Package(
    name: "mac_ddc",
    platforms: [
        .macOS(.v10_15),
    ],
    products: [
        .library(name: "mac_ddc", type: .static, targets: ["mac_ddc"]),
    ],
    dependencies: [
        .package(url: "https://github.com/reitermarkus/DDC.swift", .branch("master")),
    ],
    targets: [
        .target(name: "mac_ddc", dependencies: [.product(name: "DDC", package: "DDC.swift")], path: "src"),
    ]
)
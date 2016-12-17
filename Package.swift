import PackageDescription

let package = Package(
    name: "Mountain",
    dependencies: [
        .Package(url: "https://github.com/ReactiveCocoa/ReactiveSwift.git", "1.0.0-rc.1"),
        .Package(url: "https://github.com/kylef/PathKit.git", "0.7.1"),
    ]
)

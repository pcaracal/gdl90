// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "GDL90",
	products: [
		.library(
			name: "GDL90",
			targets: ["GDL90"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "GDL90",
            url: "https://github.com/pcaracal/gdl90/releases/download/0.0.1/RustXcframework.xcframework.zip",
			checksum: "88201060340d839a08737165f8233e8e026e97a1e8b12c11e7b0d39ab16a3c5a"
		)
	]
)

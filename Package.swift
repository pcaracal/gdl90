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
      url: "https://github.com/pcaracal/gdl90/releases/download/0.0.3/RustXcframework.xcframework.zip"
		)
	]
)

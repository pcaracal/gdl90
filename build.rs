fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi");

    swift_bridge_build::parse_bridges(["src/ffi/swift.rs"])
        .write_all_concatenated("target/swift", env!("CARGO_PKG_NAME"));
}
